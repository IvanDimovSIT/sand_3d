use std::f32::consts::PI;

use kiss3d::{nalgebra::{Quaternion, Translation, Unit, UnitQuaternion, Vector3}, scene::SceneNode, window::Window};

use crate::model::World;

const VOXEL_SIZE: f32 = 1.0;

pub struct SceneGenerator{
    origin: Translation<f32, 3>,
    left_rotation: Unit<Quaternion<f32>>,
    up_rotation: Unit<Quaternion<f32>>,
    left_translation: Translation<f32, 3>,
    right_translation: Translation<f32, 3>,
    front_translation: Translation<f32, 3>,
    back_translation: Translation<f32, 3>,
    top_translation: Translation<f32, 3>,
    bottom_translation: Translation<f32, 3>,
}
impl SceneGenerator {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let origin = Translation::from(Vector3::new(x, y, z));
        let left_rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), PI/2.0);
        let up_rotation = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), PI/2.0);
        let left_translation = Translation::from(Vector3::new(-VOXEL_SIZE/2.0, 0.0f32, 0.0f32));
        let right_translation = Translation::from(Vector3::new(VOXEL_SIZE/2.0, 0.0f32, 0.0f32));
        let front_translation = Translation::from(Vector3::new(0.0f32, 0.0f32, VOXEL_SIZE/2.0));
        let back_translation = Translation::from(Vector3::new(0.0f32, 0.0f32, -VOXEL_SIZE/2.0));
        let top_translation = Translation::from(Vector3::new(0.0f32, VOXEL_SIZE/2.0, 0.0f32));
        let bottom_translation = Translation::from(Vector3::new(0.0f32, -VOXEL_SIZE/2.0, 0.0f32));

        Self { origin, left_rotation, up_rotation, left_translation, right_translation, front_translation, back_translation, top_translation, bottom_translation }        
    }

    pub fn generate_scene(&self, window: &mut Window, world: &World) -> Vec<SceneNode> {

        todo!()
    }
}

