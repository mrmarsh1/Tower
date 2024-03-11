use ecs::Component;

use component::component;

use math::{to_double_vector, Matrix4x4, Rotation3, Vector3d, Vector3f};

#[component]
pub struct Transform {
    position: Vector3d,
    euler_angles: Vector3f,
    scale: Vector3f,
    front: Vector3d,
    right: Vector3d,
    up: Vector3d,
    matrix: Matrix4x4,
}

impl Transform {
    pub fn new(entity: usize) -> Self {
        Self {
            entity: entity,
            one_frame: false,
            position: Vector3d::zeros(),
            euler_angles: Vector3f::zeros(),
            scale: Vector3f::new(1.0, 1.0, 1.0),
            front: Vector3d::new(0.0, 0.0, -1.0),
            right: Vector3d::new(1.0, 0.0, 0.0),
            up: Vector3d::new(0.0, 1.0, 0.0),
            matrix: Matrix4x4::identity(),
        }
    }

    pub fn position(&self) -> Vector3d {
        self.position
    }

    pub fn euler_angles(&self) -> Vector3f {
        self.euler_angles
    }

    pub fn scale(&self) -> Vector3f {
        self.scale
    }

    pub fn front(&self) -> Vector3d {
        self.front
    }

    pub fn right(&self) -> Vector3d {
        self.right
    }

    pub fn up(&self) -> Vector3d {
        self.up
    }

    pub fn set_position(&mut self, value: Vector3d) {
        self.position = value;
    }

    pub fn set_euler_angles(&mut self, value: Vector3f) {
        self.euler_angles = value;
        self.update_transform();
        self.update_vectors();
    }

    pub fn set_scale(&mut self, value: Vector3f) {
        self.scale = value;
        self.update_transform();
    }

    fn update_transform(&mut self) {
        let rotation = self.euler_angles;
        let scale = Matrix4x4::new_nonuniform_scaling(&self.scale);
        self.matrix = Rotation3::from_euler_angles(rotation.x, rotation.y, rotation.z).to_homogeneous() * scale;
    }

    fn update_vectors(&mut self) {
        let world_up = Vector3f::new(0f32, 1f32, 0f32);

        let angles = self.euler_angles;
        let pitch = angles.x;
        let yaw = angles.y;

        let front = Vector3f::new(
            yaw.to_radians().cos() * pitch.to_radians().cos(),
            pitch.to_radians().sin(),
            yaw.to_radians().sin() * pitch.to_radians().cos()
        ).normalize();

        let right = front.cross(&world_up).normalize();
        let up = right.cross(&front).normalize();

        self.front = to_double_vector(front);
        self.right = to_double_vector(right);
        self.up = to_double_vector(up);
    }

    pub fn get_matrix(&self) -> Matrix4x4 {
        self.matrix
    }
}
