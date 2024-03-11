use web_sys::WebGl2RenderingContext;

use super::AppState;

pub trait App {
    fn on_init(&mut self, state: &mut AppState, context: WebGl2RenderingContext);
    fn on_tick(&mut self, state: &mut AppState);
    fn on_resize(&mut self, width: i32, height: i32);
}
