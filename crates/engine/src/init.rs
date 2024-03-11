#[macro_export]
macro_rules! init_world {
    ($($struct_name:ident),*) => {

        use $crate::ecs::init_ecs;
        use $crate::app::App;
        use $crate::webgl::WebGlRenderer;
        use $crate::Services;
        use $crate::components::*;
        use $crate::types::DrawCallInfo;
        use $crate::components::DrawPackage;
        use $crate::AssetMan;
        use $crate::WebGl2RenderingContext;

        use $crate::batch_system;
        use $crate::render_system;
        use $crate::update_camera_system;

        init_ecs! {Transform, Camera, DrawPackage, Mesh, DirectionalLight, PointLight, SpotLight, Skybox, SceneData,
            $(
                $struct_name
            ),*
        }

        pub type InitSystem = fn(services: &mut Services, state: &mut AppState, world: &mut World);
        pub type System = fn(services: &mut Services, state: &mut AppState, world: &mut World);

        pub struct Engine {
            world: World,
            init_systems: Vec<InitSystem>,
            tick_systems: Vec<System>,
            services: Services,
        }

        impl Engine {
            pub fn new() -> Self {
                let mut world = World::create();
                SceneData {
                    entity: world.create_entity(),
                    one_frame: false,
                    ambient: Color::new(0.2, 0.2, 0.2),
                    env: None,
                }.add(&mut world);

                Engine {
                    world: world,
                    init_systems: vec![
                    ],
                    tick_systems: vec![
                        update_camera_system!{},
                        batch_system!{},
                        render_system!{},
                    ],
                    services: Services::new(),
                }
            }

            pub fn register_init(&mut self, system: InitSystem) {
                self.init_systems.push(system);
            }

            pub fn register_tick(&mut self, system: System) {
                self.tick_systems.push(system);
            }
        }

        impl App for Engine {
            fn on_init(&mut self, state: &mut AppState, context: WebGl2RenderingContext) {
                let renderer = WebGlRenderer::new(context);
                self.services.add_service(renderer);
            }

            fn on_tick(&mut self, state: &mut AppState) {
                for system in self.init_systems.iter() {
                    system(&mut self.services, state, &mut self.world);
                }

                self.init_systems.clear();

                for system in self.tick_systems.iter() {
                    system(&mut self.services, state, &mut self.world);
                }

                self.world.clear_one_frame();
            }

            fn on_resize(&mut self, width: i32, height: i32) {
                self.services.resolve::<WebGlRenderer>().unwrap().set_viewport(width, height);
            }
        }
    };
}
