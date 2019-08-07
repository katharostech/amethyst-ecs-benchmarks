use nalgebra::{Isometry3, Matrix4, Vector3};
use amethyst_core::ecs::{Component, VecStorage};

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

pub struct LegionParent {
    pub parent: legion::Entity,
}

pub struct SpecsParent {
    pub specs: amethyst_core::ecs::Entity,
}
impl Component for SpecsParent {
    type Storage = VecStorage<Self>;
}
