use std::rc::Rc;

use component::component;
use loader::tower_app;

use ecs::Component;
use engine::components::{Camera, Mesh, Transform};
use engine::init_world;
use engine::types::{Material, MeshData, Texture, Tlu};
use math::{Color, Vector3d, Vector3f};
use webapp::app::{AppState, Window};

init_world! {Cube}

#[component]
struct Cube {
    spin_speed: f32,
}

#[tower_app]
async fn start() -> Result<(), JsValue> {
    let mut engine = Engine::new();

    let asset_man = AssetMan::init(vec![
        "./assets/meshes/cube.tmf",
        "./assets/textures/checker.png",
        "./assets/textures/checker_spec.png",
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
    engine.register_tick(rotate_cube);

    Window::new(engine);
    Ok(())
}

fn init_scene(services: &mut Services, state: &mut AppState, world: &mut World) {
    let asset_man = services.resolve::<AssetMan>().unwrap();
    let renderer = services.resolve::<WebGlRenderer>().unwrap();

    let shader_lit: Rc<Tlu> = asset_man
        .get_asset("./assets/shaders/lit.tlu".to_string())
        .unwrap();

    let mesh_data: Rc<MeshData> = asset_man
        .get_asset("./assets/meshes/cube.tmf".to_string())
        .unwrap();

    let texture = asset_man
        .get_asset::<Texture>("./assets/textures/checker.png".to_string())
        .unwrap();

    let dimensions = texture.get_dimensions();
    let texture_id = renderer
        .create_texture(texture.get_data(), dimensions.0, dimensions.1)
        .texture();

    let texture_spec = asset_man
        .get_asset::<Texture>("./assets/textures/checker_spec.png".to_string())
        .unwrap();

    let dimensions = texture.get_dimensions();
    let texture_spec_id = renderer
        .create_texture(texture_spec.get_data(), dimensions.0, dimensions.1)
        .texture();

    let program_id = renderer
        .create_program(shader_lit.vert(), shader_lit.frag())
        .program();

    let mesh_id = renderer
        .create_mesh(
            mesh_data.raw_vertices(),
            mesh_data.raw_indices(),
            webgl::DrawMode::Static,
        )
        .vao();

    for e in world.query().scenedata().fetch() {
        let scene_data = world.get_scenedata(e).unwrap();
        scene_data.ambient = Color::new(0.1, 0.1, 0.1);
    }

    // components

    let camera_entity = world.create_entity();
    let camera = Camera::new(camera_entity);

    camera.add(world);

    let mut transform = Transform::new(camera_entity);
    transform.set_position(Vector3d::new(0.0, 1.8, 3.0));
    transform.set_euler_angles(Vector3f::new(-25.0, -90.0, 0.0));
    transform.add(world);

    let cube_entity = world.create_entity();

    let material = Rc::new(Material::new(
        shader_lit,
        program_id,
        32.0,
        Some(texture_id),
        Some(texture_spec_id),
    ));

    Mesh {
        entity: cube_entity,
        one_frame: false,
        mesh_data: mesh_data,
        mesh_id,
        material: Some(Rc::clone(&material)),
    }
    .add(world);

    let mut transform = Transform::new(cube_entity);
    transform.set_position(Vector3d::new(0.0, 0.0, 0.0));
    transform.set_scale(Vector3f::new(2.0, 1.0, 2.0));
    transform.add(world);

    Cube {
        entity: cube_entity,
        one_frame: false,
        spin_speed: 45f32,
    }
    .add(world);

    // -- lights

    let light_entity = world.create_entity();

    PointLight {
        entity: light_entity,
        one_frame: false,
        diffuse: Color::new(1.0, 1.0, 1.0),
        specular: Color::new(1.0, 1.0, 1.0),
        constant: 1.0 / 3.0,
        linear: 0.09,
        quadratic: 0.032,
    }
    .add(world);

    let mut transform = Transform::new(light_entity);
    transform.set_position(Vector3d::new(-1.0, 2.0, -1.0));
    transform.add(world);
}

fn rotate_cube(services: &mut Services, state: &mut AppState, world: &mut World) {
    for e in world.query().mesh().transform().cube().fetch() {
        let transform = world.get_transform(e).unwrap();
        let cube = world.get_cube(e).unwrap();
        transform.set_euler_angles(
            transform.euler_angles()
                + Vector3f::new(
                    0.0f32,
                    -cube.spin_speed.to_radians() * state.delta_time(),
                    0.0f32,
                ),
        );
    }
}
