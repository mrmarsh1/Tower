use std::rc::Rc;

use component::component;
use loader::tower_app;

use ecs::Component;
use engine::components::{Camera, Mesh, Transform};
use engine::init_world;
use engine::types::{Material, MeshData, Texture, Tlu};
use math::{Color, Vector3d, Vector3f};
use webapp::app::{AppState, Window};

#[component]
struct CameraInput {
    x_rel: i32,
    y_rel: i32,
    forward: i32,
    right: i32,
}

#[component]
struct Aircraft {}

init_world! {CameraInput, Aircraft}

#[tower_app]
async fn start() -> Result<(), JsValue> {
    let mut engine = Engine::new();

    let asset_man = AssetMan::init(vec![
        "./assets/shaders/unlit.tlu",
        "./assets/shaders/skybox.tlu",
        "./assets/meshes/aircraft.tmf",
        "./assets/textures/aircraft.png",
        "./assets/textures/aircraft_spec.png",
        "./assets/textures/sky/x_pos.png",
        "./assets/textures/sky/x_neg.png",
        "./assets/textures/sky/y_pos.png",
        "./assets/textures/sky/y_neg.png",
        "./assets/textures/sky/z_pos.png",
        "./assets/textures/sky/z_neg.png",
    ])
    .await?;

    asset_man
        .load_shader(
            "./assets/shaders/lit.tlu",
            Some(vec![
                "./assets/shaders/material.glsl",
                "./assets/shaders/lighting.glsl",
            ]),
        )
        .await?;

    engine.services.add_service(asset_man);

    engine.register_init(init_scene);
    engine.register_tick(process_input);
    engine.register_tick(rotate_camera);
    engine.register_tick(move_plane_lissajous);
    // engine.register_tick(|_, _, w| {
    //     utils::log(format!("{:?}", w.stat()).as_str());
    // });

    Window::new(engine);
    Ok(())
}

fn init_scene(services: &mut Services, state: &mut AppState, world: &mut World) {
    let asset_man = services.resolve::<AssetMan>().unwrap();
    let renderer = services.resolve::<WebGlRenderer>().unwrap();

    let shader_lit: Rc<Tlu> = asset_man
        .get_asset("./assets/shaders/lit.tlu".to_string())
        .unwrap();

    let aircraft_mesh_data: Rc<MeshData> = asset_man
        .get_asset("./assets/meshes/aircraft.tmf".to_string())
        .unwrap();

    let aircraft_texture = asset_man
        .get_asset::<Texture>("./assets/textures/aircraft.png".to_string())
        .unwrap();

    let aircraft_texture_specular = asset_man
        .get_asset::<Texture>("./assets/textures/aircraft_spec.png".to_string())
        .unwrap();

    let program_id = renderer
        .create_program(shader_lit.vert(), shader_lit.frag())
        .program();

    let mesh_id = renderer.create_mesh(
        aircraft_mesh_data.raw_vertices(),
        aircraft_mesh_data.raw_indices(),
        webgl::DrawMode::Static,
    ).vao();

    let dimensions = aircraft_texture.get_dimensions();
    let texture_id = renderer
        .create_texture(aircraft_texture.get_data(), dimensions.0, dimensions.1)
        .texture();

    let dimensions = aircraft_texture_specular.get_dimensions();
    let spec_texture_id = renderer
        .create_texture(
            aircraft_texture_specular.get_data(),
            dimensions.0,
            dimensions.1,
        )
        .texture();

    let x_pos = asset_man
        .get_asset::<Texture>("./assets/textures/sky/x_pos.png".to_string())
        .unwrap();

    let x_neg = asset_man
        .get_asset::<Texture>("./assets/textures/sky/x_neg.png".to_string())
        .unwrap();

    let y_pos = asset_man
        .get_asset::<Texture>("./assets/textures/sky/y_pos.png".to_string())
        .unwrap();

    let y_neg = asset_man
        .get_asset::<Texture>("./assets/textures/sky/y_neg.png".to_string())
        .unwrap();

    let z_pos = asset_man
        .get_asset::<Texture>("./assets/textures/sky/z_pos.png".to_string())
        .unwrap();

    let z_neg = asset_man
        .get_asset::<Texture>("./assets/textures/sky/z_neg.png".to_string())
        .unwrap();

    let skybox_shader: Rc<Tlu> = asset_man
        .get_asset("./assets/shaders/skybox.tlu".to_string())
        .unwrap();

    let skybox_texture = Rc::new(
        renderer
            .create_cube_texture(
                vec![
                    &x_neg.data,
                    &x_pos.data,
                    &y_pos.data,
                    &y_neg.data,
                    &z_pos.data,
                    &z_neg.data,
                ],
                x_pos.width,
                x_pos.height,
            )
            .texture(),
    );

    for e in world.query().scenedata().fetch() {
        let scene_data = world.get_scenedata(e).unwrap();
        scene_data.ambient = Color::new(0.1, 0.1, 0.1);
        scene_data.env = Some(Rc::clone(&skybox_texture));
    }

    // components

    let camera_entity = world.create_entity();
    let camera = Camera::new(camera_entity);

    camera.add(world);

    let mut transform = Transform::new(camera_entity);
    transform.set_position(Vector3d::new(-3.0, 4.5, -11.0));
    transform.set_euler_angles(Vector3f::new(-25.0f32, 80.0f32, 0.0f32));
    transform.add(world);

    let aircraft_entity = world.create_entity();

    let material = Rc::new(Material::new(
        shader_lit,
        program_id,
        32.0,
        Some(texture_id),
        Some(spec_texture_id),
    ));

    Mesh {
        entity: aircraft_entity,
        one_frame: false,
        mesh_data: aircraft_mesh_data,
        mesh_id,
        material: Some(Rc::clone(&material)),
    }
    .add(world);

    let mut transform = Transform::new(aircraft_entity);
    transform.set_position(Vector3d::new(0.0, 0.0, 0.0));
    transform.add(world);

    Aircraft {
        entity: aircraft_entity,
        one_frame: false,
    }
    .add(world);

    // -- lights

    let light_entity = world.create_entity();

    DirectionalLight {
        entity: light_entity,
        one_frame: false,
        diffuse: Color::new(0.3, 0.3, 0.3),
        specular: Color::new(0.3, 0.3, 0.3),
    }
    .add(world);

    let mut transform = Transform::new(light_entity);
    transform.set_euler_angles(Vector3f::new(-90.0, 0.0, 0.0));
    transform.add(world);

    // -----

    let skybox_program_id = renderer.create_program(skybox_shader.vert(), skybox_shader.frag()).program();
    let skybox_mesh_data = Skybox::get_mesh_data();
    let skybox_vao = renderer.create_skybox_mesh(&skybox_mesh_data);
    Skybox::create(
        world.create_entity(),
        skybox_shader,
        skybox_program_id,
        skybox_vao,
        skybox_mesh_data.len() as i32,
        Rc::clone(&skybox_texture),
    )
    .add(world);
}

fn process_input(services: &mut Services, state: &mut AppState, world: &mut World) {
    let mut forward = 0;
    let mut right = 0;

    if state.is_pressed("KeyW") {
        forward = 1;
    } else if state.is_pressed("KeyS") {
        forward = -1;
    }

    if state.is_pressed("KeyA") {
        right = -1;
    } else if state.is_pressed("KeyD") {
        right = 1;
    }

    let mouse_delta = state.get_mouse_delta();

    CameraInput {
        entity: world.create_entity(),
        one_frame: true,
        x_rel: mouse_delta.0,
        y_rel: mouse_delta.1,
        forward,
        right,
    }
    .add(world);
}

const MOUSE_SENSITIVITY: f32 = 0.1;
const MOVEMENT_SPEED: f64 = 5.0;

fn rotate_camera(services: &mut Services, state: &mut AppState, world: &mut World) {
    for e in world.query().camerainput().fetch() {
        let input = world.get_camerainput(e).unwrap();
        for e in world.query().camera().transform().fetch() {
            let transform = world.get_transform(e).unwrap();

            transform.set_euler_angles(
                transform.euler_angles()
                    + Vector3f::new(
                        -input.y_rel as f32 * MOUSE_SENSITIVITY,
                        input.x_rel as f32 * MOUSE_SENSITIVITY,
                        0.0f32,
                    ),
            );

            if transform.euler_angles().x > 89.0 {
                let mut angles = transform.euler_angles();
                angles.x = 89.0;
                transform.set_euler_angles(angles);
            } else if transform.euler_angles().x < -89.0 {
                let mut angles = transform.euler_angles();
                angles.x = -89.0;
                transform.set_euler_angles(angles);
            }

            transform.set_position(
                transform.position()
                    + transform
                        .front()
                        .scale(input.forward as f64 * MOVEMENT_SPEED * state.delta_time() as f64),
            );
            transform.set_position(
                transform.position()
                    + transform
                        .right()
                        .scale(input.right as f64 * MOVEMENT_SPEED * state.delta_time() as f64),
            );
        }
    }
}

fn move_plane_lissajous(services: &mut Services, state: &mut AppState, world: &mut World) {
    for e in world.query().aircraft().transform().fetch() {
        let transform = world.get_transform(e).unwrap();

        let a1 = 0.5f32;
        let b1 = 0.5f32;
        let a2 = 1.0f32;
        let b2 = 1.5f32;
        let delta = std::f32::consts::PI / 2.0;

        let t = state.time() as f32 * 0.001;

        let x = a1 * (a2 * t + delta).sin();
        let y = b1 * (b2 * t).sin();

        transform.set_position(Vector3d::new(x as f64, y as f64, 0.0));

        let angle = (state.time() * 0.0002).sin() * 0.25;

        transform.set_euler_angles(Vector3f::new(0.0, 0.0, angle as f32));
    }
}
