extern crate kiss3d;

mod model;
mod scene_generator;
mod texture_generator;
mod wire_cube;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use kiss3d::nalgebra::{Point3, Translation, UnitQuaternion, Vector3};
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::resource::{Material, MaterialManager, Mesh, TextureManager};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use kiss3d::light::Light;
use model::{World, WORLD_SIZE};
use scene_generator::{SceneGenerator, VOXEL_SIZE};

const FPS: u64 = 60;
const ORIGIN_X: f32 = 0.0;
const ORIGIN_Y: f32 = 0.0;
const ORIGIN_Z: f32 = 0.0;

//const LIGHT_POSITION_X: f32 = ORIGIN_X + VOXEL_SIZE * WORLD_SIZE as f32 / 3.0;
//const LIGHT_POSITION_Y: f32 = ORIGIN_Y + VOXEL_SIZE * WORLD_SIZE as f32 * 1.2;
//const LIGHT_POSITION_Z: f32 = ORIGIN_Z + VOXEL_SIZE * WORLD_SIZE as f32 / 2.0;

fn main() {
    let mut window = Window::new("Sand 3D");
    window.set_framerate_limit(Some(FPS));
    window.set_background_color(0.8, 0.8, 0.9);
    
    //window.set_light(Light::Absolute(Point3::new(LIGHT_POSITION_X, LIGHT_POSITION_Y, LIGHT_POSITION_Z)));
    window.set_light(Light::StickToCamera);

    let mut world = World::new();
    world.set(model::VoxelMaterial::Sand, 2, 7, 2); // Initial setup
    world.set(model::VoxelMaterial::Sand, 3, 7, 2); // Initial setup
    world.set(model::VoxelMaterial::Sand, 2, 8, 2); // Initial setup
    world.set(model::VoxelMaterial::Water, 3, 9, 3);

    let scene_generator = SceneGenerator::new(ORIGIN_X, ORIGIN_Y, ORIGIN_Z);
    let mut should_generate = true;
    let mut scene_nodes: Vec<SceneNode> = vec![]; 
    while window.render() {
        scene_generator.draw_border(&mut window);
        if should_generate {
            should_generate = false;
            for i in &mut scene_nodes {
                window.remove_node(i);
            }
            scene_nodes = scene_generator.generate_scene(&mut window, &world);
            println!("generating {} objects", scene_nodes.len());
        }

        should_generate = true; // TODO: Add update logic

    }
}
