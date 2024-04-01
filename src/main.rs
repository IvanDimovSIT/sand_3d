extern crate kiss3d;

mod model;
mod scene_generator;
mod texture_generator;
mod wire_cube;
mod material_properties;
mod voxel_simulator;
mod material_reactions;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::time::{Duration, Instant};

use kiss3d::nalgebra::{Point3, Translation, UnitQuaternion, Vector3};
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::resource::{Material, MaterialManager, Mesh, TextureManager};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use kiss3d::light::Light;
use model::{World, WORLD_SIZE};
use scene_generator::{SceneGenerator, VOXEL_SIZE};
use voxel_simulator::VoxelSimulator;

const FPS: u64 = 60;
const ORIGIN_X: f32 = -VOXEL_SIZE * WORLD_SIZE as f32 / 2.0;
const ORIGIN_Y: f32 = -VOXEL_SIZE * WORLD_SIZE as f32 * 1.2;
const ORIGIN_Z: f32 = 50.0;

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
    let mut voxel_simulator = VoxelSimulator::new();
    world.set(model::VoxelMaterial::Sand, 2, 7, 2); // Initial setup
    world.set(model::VoxelMaterial::Sand, 3, 7, 2); // Initial setup
    world.set(model::VoxelMaterial::Sand, 2, 8, 2); // Initial setup
    world.set(model::VoxelMaterial::Water, 3, 9, 3);
    world.set(model::VoxelMaterial::Metal, 3, 0, 4);

    let scene_generator = SceneGenerator::new(ORIGIN_X, ORIGIN_Y, ORIGIN_Z);
    let mut should_generate = true;
    let mut scene_nodes: Vec<SceneNode> = vec![]; 
    let mut total_render_time = Duration::new(0, 0);
    let mut start_time = Instant::now();
    while window.render() {
        total_render_time += start_time.elapsed();

        scene_generator.draw_border(&mut window);
        if should_generate {
            should_generate = false;
            for i in &mut scene_nodes {
                window.remove_node(i);
            }
            scene_nodes = scene_generator.generate_scene(&mut window, &world);
            println!("generating {} objects", scene_nodes.len());
        }
        world.set(model::VoxelMaterial::SaltWater, 1, WORLD_SIZE-1, 1);
        world.set(model::VoxelMaterial::Salt, WORLD_SIZE-3, WORLD_SIZE-1, WORLD_SIZE-3);
        
        if total_render_time.as_millis() > 80 {
            should_generate = voxel_simulator.next_step(&mut world);
            total_render_time = Duration::new(0, 0);
        }
        start_time = Instant::now();
    }
}
