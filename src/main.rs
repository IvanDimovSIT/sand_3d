extern crate kiss3d;

mod model;
mod scene_generator;
mod texture_generator;
mod wire_cube;
mod material_properties;
mod voxel_simulator;
mod material_reactions;
mod scene_map;
mod cursor;

use std::time::Instant;

use cursor::Cursor;
use kiss3d::camera::FirstPerson;
use kiss3d::event::Key;
use kiss3d::nalgebra::{Point, Point3};
use kiss3d::event::WindowEvent;
use kiss3d::window::Window;
use kiss3d::light::Light;
use model::{World, WORLD_SIZE};
use scene_generator::{SceneGenerator, VOXEL_SIZE};
use scene_map::SceneMap;
use voxel_simulator::VoxelSimulator;

const FPS: u64 = 60;
const ORIGIN_X: f32 = -VOXEL_SIZE * WORLD_SIZE as f32 / 2.0;
const ORIGIN_Y: f32 = -VOXEL_SIZE * WORLD_SIZE as f32 * 1.5;
const ORIGIN_Z: f32 = 85.0;
const TIME_BETWEEN_STEPS_US: u128 = 50_000;

fn move_camera(key: Key, eye_x: &mut f32, eye_y: f32, eye_z: &mut f32, at: Point<f32, 3>, fp_camera: &mut FirstPerson) {
    match key {
        Key::W => { *eye_z += VOXEL_SIZE },
        Key::S => { *eye_z -= VOXEL_SIZE },
        Key::A => { *eye_x += VOXEL_SIZE },
        Key::D => { *eye_x -= VOXEL_SIZE },
        _ => {return;},
    }

    *fp_camera = FirstPerson::new(Point3::new(*eye_x, eye_y, *eye_z), at);
}

fn main() {
    let mut window = Window::new("Sand 3D");
    window.set_framerate_limit(Some(FPS));
    window.set_background_color(0.8, 0.8, 0.9);

    let (mut eye_x, eye_y, mut eye_z) = (0.0, 0.0, 0.0);
    let eye = Point3::new(eye_x, eye_y, eye_z);
    let at = Point3::new(
        ORIGIN_X+WORLD_SIZE as f32 * VOXEL_SIZE/2.0,
        ORIGIN_Y+WORLD_SIZE as f32 * VOXEL_SIZE/3.0,
        ORIGIN_Z+WORLD_SIZE as f32 * VOXEL_SIZE/2.0
    );
    let mut fp_camera = FirstPerson::new(eye, at);
    
    window.set_light(Light::StickToCamera);

    let mut world = World::new();
    let mut voxel_simulator = VoxelSimulator::new();
    let scene_generator = SceneGenerator::new(ORIGIN_X, ORIGIN_Y, ORIGIN_Z);
    let mut paused = false;
    let mut scene_map = SceneMap::new();
    let mut cursor = Cursor::new(WORLD_SIZE/2, WORLD_SIZE-1, WORLD_SIZE/2);
    let mut render_time;
    let mut generation_time = 0;
    let mut simulation_time = 0;
    let mut render_start_time = Instant::now();
    let mut generation_start_time;
    let mut simulation_start_time;
    let mut total_render_time = 0;
    while window.render_with_camera(&mut fp_camera) {
        render_time = render_start_time.elapsed().as_micros();
        total_render_time += render_time;
        scene_generator.draw_border(&mut window);

        generation_start_time = Instant::now();
        let changed = scene_map.get_and_remove_changed(&mut window);
        scene_generator.generate_scene(&mut window, &world, &mut scene_map, changed);    
        generation_time = generation_start_time.elapsed().as_micros();

        for mut e in window.events().iter() {
            e.inhibited = true;
            match e.value {
                WindowEvent::Key(key, action, _modif) => {
                    cursor.input_key(key, action);
                    match key {
                        kiss3d::event::Key::Space => if matches!(action, kiss3d::event::Action::Press) { paused = !paused},
                        _ => {move_camera(key, &mut eye_x, eye_y, &mut eye_z, at, &mut fp_camera)},
                    }
                },
                WindowEvent::MouseButton(button, _action, _modif) => {
                    cursor.input_click(&mut world, &mut scene_map, button);
                },
                WindowEvent::CursorPos(x, y, _modif) => {
                    cursor.input_move(1.0-x/window.width() as f64, 1.0-y/window.height() as f64);
                },
                WindowEvent::Scroll(_a, b, _modif) => {
                    cursor.input_scroll(b);
                },
                _ => {e.inhibited = false;}
            }
        }
        
        if total_render_time > TIME_BETWEEN_STEPS_US {
            if !paused {
                simulation_start_time = Instant::now();
                voxel_simulator.next_step(&mut world, &mut scene_map);
                simulation_time = simulation_start_time.elapsed().as_micros();
            }
            total_render_time -= TIME_BETWEEN_STEPS_US;
        }

        println!(
            "Render time: {}ms {}us; Mesh generation time: {}ms {}us; Simulation time: {}ms {}us",
            render_time/1000, render_time%1000,
            generation_time/1000, generation_time%1000,
            simulation_time/1000, simulation_time%1000
        );
        
        cursor.draw(&mut window, ORIGIN_X, ORIGIN_Y, ORIGIN_Z);
        cursor.draw_selected(&mut window);

        simulation_time = 0;
        generation_time = 0;
        render_start_time = Instant::now();
    }
}
