use nalgebra;

pub type Vector3f = nalgebra::Vector3<f32>;
pub type Vector2f = nalgebra::Vector2<f32>;
pub type Vector3d = nalgebra::Vector3<f64>;
pub type Matrix4x4 = nalgebra::Matrix4<f32>;
pub type Rotation3 = nalgebra::Rotation3<f32>;
pub type Color = nalgebra::Vector3<f32>;
pub type Point3 = nalgebra::Point3<f32>;

pub fn to_double_vector(vec: Vector3f) -> Vector3d {
    Vector3d::new(vec.x as f64, vec.y as f64, vec.z as f64)
}

pub fn to_single_vector(vec: Vector3d) -> Vector3f {
    Vector3f::new(vec.x as f32, vec.y as f32, vec.z as f32)
}

pub fn split_double(value: Vector3d) -> (Vector3f, Vector3f) {
    let x_1 = value.x as f32;
    let x_2 = (value.x - (x_1 as f64)) as f32;

    let y_1 = value.y as f32;
    let y_2 = (value.y - (y_1 as f64)) as f32;

    let z_1 = value.z as f32;
    let z_2 = (value.z - (z_1 as f64)) as f32;

    (
        Vector3f::new(x_1, y_1, z_1),
        Vector3f::new(x_2, y_2, z_2),
    )
}
