use nalgebra::{Isometry3, Vector3, Matrix4};
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Transform {
    pub isometry: Isometry3<f32>,
    pub scale: Vector3<f32>,
    pub global_matrix: Matrix4<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            isometry: Isometry3::identity(),
            scale: Vector3::identity(),
            global_matrix: Matrix4::identity(),
        }
    }
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
