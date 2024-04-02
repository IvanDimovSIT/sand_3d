extern crate kiss3d;

mod model;
mod scene_generator;
mod texture_generator;
mod wire_cube;
mod material_properties;
mod voxel_simulator;
mod material_reactions;
mod scene_map;


use std::time::Instant;

use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use kiss3d::light::Light;
use model::{World, WORLD_SIZE};
use scene_generator::{SceneGenerator, VOXEL_SIZE};
use scene_map::SceneMap;
use voxel_simulator::VoxelSimulator;

const FPS: u64 = 60;
const ORIGIN_X: f32 = -VOXEL_SIZE * WORLD_SIZE as f32 / 2.0;
const ORIGIN_Y: f32 = -VOXEL_SIZE * WORLD_SIZE as f32 * 1.2;
const ORIGIN_Z: f32 = 50.0;
const TIME_BETWEEN_STEPS_US: u128 = 80_000;

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
    //world.set(model::VoxelMaterial::Sand, 2, 7, 2); // Initial setup
    //world.set(model::VoxelMaterial::Sand, 3, 7, 2); // Initial setup
    //world.set(model::VoxelMaterial::Sand, 2, 8, 2); // Initial setup
    //world.set(model::VoxelMaterial::Water, 3, 9, 3);
    //world.set(model::VoxelMaterial::Metal, 3, 0, 4);

    let scene_generator = SceneGenerator::new(ORIGIN_X, ORIGIN_Y, ORIGIN_Z);
    let mut scene_map = SceneMap::new();

    let mut render_time;
    let mut generation_time = 0;
    let mut simulation_time = 0;
    let mut render_start_time = Instant::now();
    let mut generation_start_time;
    let mut simulation_start_time;
    let mut total_render_time = 0;
    while window.render() {
        render_time = render_start_time.elapsed().as_micros();
        total_render_time += render_time;

        scene_generator.draw_border(&mut window);

        generation_start_time = Instant::now();
        let changed = scene_map.get_and_remove_changed(&mut window);
        scene_generator.generate_scene(&mut window, &world, &mut scene_map, changed);    
        generation_time = generation_start_time.elapsed().as_micros();

        world.set(model::VoxelMaterial::Water, 1, WORLD_SIZE-1, 1);
        world.set(model::VoxelMaterial::Salt, WORLD_SIZE-3, WORLD_SIZE-1, WORLD_SIZE-3);
        world.set(model::VoxelMaterial::Lava, 1, WORLD_SIZE-1, WORLD_SIZE/2);
        
        if total_render_time > TIME_BETWEEN_STEPS_US {
            simulation_start_time = Instant::now();
            voxel_simulator.next_step(&mut world, &mut scene_map);
            simulation_time = simulation_start_time.elapsed().as_micros();
            total_render_time -= TIME_BETWEEN_STEPS_US;
        }

        println!(
            "Render time: {}ms {}us; Mesh generation time: {}ms {}us; Simulation time: {}ms {}us",
            render_time/1000, render_time%1000,
            generation_time/1000, generation_time%1000,
            simulation_time/1000, simulation_time%1000
        );
        simulation_time = 0;
        generation_time = 0;
        render_start_time = Instant::now();
    }
}
