## Tower
WebGL 3d renderer written in Rust.

### Example scene
[![Aircraft](/examples/aircraft/aircraft.png)](https://laykku.github.io/store.github.io/aircraft/index.html)

### Example code
Init ECS:

```
#[component]
struct Cube {
    spin_speed: f32,
}

init_world! {Cube}
```

Create cube
```
let cube_entity = world.create_entity();

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
```

Write system:

```
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
```

Register system:

```
engine.register_tick(rotate_cube);
```

Full example: [Simple scene](/examples/simple_scene/)

### Mesh exporter plugin for Blender
[TMF exporter](/tmf)
