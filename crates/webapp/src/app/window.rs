use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use wasm_bindgen::prelude::*;
use web_sys::{Performance, WebGl2RenderingContext};

use super::{state::AppState, App};

pub struct Window {
    window: web_sys::Window,
    performace: Rc<Performance>,
    document: web_sys::Document,
    canvas: web_sys::HtmlCanvasElement,
    state: Arc<Mutex<Rc<RefCell<AppState>>>>,
    app: Arc<Mutex<dyn App>>,
}

impl Window {
    pub fn new<T: App + 'static>(app: T) {
        let window = Window {
            window: Window::get_window(),
            performace: Rc::new(Window::get_window().performance().unwrap()),
            document: Window::get_document(),
            canvas: Window::get_canvas(),
            state: Arc::new(Mutex::new(Rc::new(RefCell::new(AppState::new())))),
            app: Arc::new(Mutex::new(app)),
        };

        window.app.lock().unwrap().on_init(&mut *window.state.lock().unwrap().borrow_mut(), Self::get_context());
        window.update_canvas_size();

        window.setup_onresize_event();
        window.setup_canvas_click_event();
        window.setup_mouse_events();
        window.setup_keyboard_callbacks();
        window.start_render_loop();
    }

    fn update_canvas_size(&self) {
        let width = self.canvas.client_width();
        let height = self.canvas.client_height();
        self.state.lock().unwrap().borrow_mut().resolution = (width, height);
        self.canvas.set_attribute("width", format!("{}", width).as_str()).unwrap();
        self.canvas.set_attribute("height", format!("{}", height).as_str()).unwrap();
        self.app.lock().unwrap().on_resize(width, height);
    }

    fn get_window() -> web_sys::Window {
        web_sys::window().unwrap()
    }

    fn get_context() -> WebGl2RenderingContext {
        Self::get_canvas()
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap()
    }

    fn get_document() -> web_sys::Document {
        Window::get_window().document().unwrap()
    }

    fn get_canvas() -> web_sys::HtmlCanvasElement {
        let canvas = Window::get_document()
            .get_element_by_id("app-canvas")
            .unwrap();
        canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("error initializing window")
    }

    fn setup_onresize_event(&self) {
        let canvas = Self::get_canvas();
        let app = Arc::clone(&self.app);
        let state = Arc::clone(&self.state);
        let window_resize_closure =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                let width = canvas.client_width();
                let height = canvas.client_height();
                state.lock().unwrap().borrow_mut().resolution = (width, height);
                canvas.set_attribute("width", format!("{}", width).as_str()).unwrap();
                canvas.set_attribute("height", format!("{}", height).as_str()).unwrap();
                app.lock().unwrap().on_resize(width, height);
            });
        self.window
            .add_event_listener_with_callback(
                "resize",
                window_resize_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        window_resize_closure.forget();
    }

    fn setup_keyboard_callbacks(&self) {
        let state = Arc::clone(&self.state);
        let key_press_closure =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
                state
                    .lock()
                    .unwrap()
                    .borrow_mut()
                    .handle_key_press(&event.code());
                //utils::log(format!("{}", event.code()).as_str());
            });
        self.document
            .add_event_listener_with_callback(
                "keypress",
                key_press_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        key_press_closure.forget();

        let state = Arc::clone(&self.state);
        let key_release_closure =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
                state
                    .lock()
                    .unwrap()
                    .borrow_mut()
                    .handle_key_release(&event.code());
            });
        self.document
            .add_event_listener_with_callback("keyup", key_release_closure.as_ref().unchecked_ref())
            .unwrap();
        key_release_closure.forget();
    }

    fn setup_mouse_events(&self) {
        let state = Arc::clone(&self.state);
        let mouse_press_closure =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                state
                    .lock()
                    .unwrap()
                    .borrow_mut()
                    .handle_mouse_press(event.button());
            });
        self.canvas
            .add_event_listener_with_callback(
                "mousedown",
                mouse_press_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        mouse_press_closure.forget();

        let state = Arc::clone(&self.state);
        let mouse_release_closure =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                state
                    .lock()
                    .unwrap()
                    .borrow_mut()
                    .handle_mouse_release(event.button());
            });
        self.canvas
            .add_event_listener_with_callback(
                "mouseup",
                mouse_release_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        mouse_release_closure.forget();

        let state = Arc::clone(&self.state);
        let mouse_move_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            if let Ok(state) = state.lock() {
                let mut state = state.borrow_mut();
                state.mouse_pos = (event.offset_x(), event.offset_y());
                state.mouse_delta = (event.movement_x(), event.movement_y());
            }
        });

        self.canvas
            .add_event_listener_with_callback(
                "mousemove",
                mouse_move_closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        mouse_move_closure.forget();

        let state = Arc::clone(&self.state);
        let mouse_scroll_closure =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseScrollEvent| {
                state.lock().unwrap().borrow_mut().mouse_scroll = event.axis();
            });
        self.canvas
            .add_event_listener_with_callback(
                "wheel",
                mouse_scroll_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        mouse_scroll_closure.forget();
    }

    fn setup_canvas_click_event(&self) {
        let canvas_clone = self.canvas.clone();
        let canvas_click_closure =
            Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                canvas_clone.request_pointer_lock();
            });
        self.canvas
            .add_event_listener_with_callback(
                "click",
                canvas_click_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        canvas_click_closure.forget();
    }

    fn start_render_loop(&self) {
        let state = Arc::clone(&self.state);
        let performance = Rc::clone(&self.performace);

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let app = Arc::clone(&self.app);
        *g.borrow_mut() = Some(Closure::new(move || {
            
            let state = state.lock().unwrap();
            let state = &mut *state.borrow_mut();

            let now = performance.now();
            state.time = now;
            state.delta_time = ((now - state.last_tick_time) / 1000.0) as f32;
            state.last_tick_time = now;

            if false {
                let _ = f.borrow_mut().take();
                return;
            }

            app.lock().unwrap().on_tick(state);

            state.mouse_delta = (0, 0);

            Window::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        Window::request_animation_frame(g.borrow().as_ref().unwrap());
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        web_sys::window()
            .expect("no global 'window' exists")
            .request_animation_frame(f.as_ref().unchecked_ref())
            .unwrap();
    }
}
