#[macro_export]
macro_rules! update_camera_system {
    () => {
        |services: &mut Services, state: &mut AppState, world: &mut World| {
            for e in world.query().camera().transform().fetch() {
                let camera = world.get_camera(e).unwrap();
                let transform = world.get_transform(e).unwrap();

                let angles = transform.euler_angles();

                let position = math::Point3::new(0f32, 0f32, 0f32);

                let target = position + math::to_single_vector(transform.front());
                let up = math::to_single_vector(transform.up());
                camera.set_view_matrix(math::Matrix4x4::look_at_rh(&position, &target, &up));
            }
        }
    };
}
