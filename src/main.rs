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
mod camera;

use std::time::Instant;

use camera::Camera;
use cursor::Cursor;
use kiss3d::event::{Action, Key};
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

const CAMERA_MOVEMENT_SPEED: f32 = 5e-5;

fn print_time(render_time: u128, generation_time: u128, simulation_time: u128) {
    let total_time = render_time + generation_time + simulation_time;
    println!(
        "Render time: {}ms {}us; Mesh generation time: {}ms {}us; Simulation time: {}ms {}us; Total: {}ms {}us",
        render_time/1000, render_time%1000,
        generation_time/1000, generation_time%1000,
        simulation_time/1000, simulation_time%1000,
        total_time/1000, total_time%1000
    );
}

fn main() {
    let mut window = Window::new("Sand 3D");
    window.set_framerate_limit(Some(FPS));
    window.set_background_color(0.8, 0.8, 0.9);
    window.hide_cursor(true);

    let mut camera = Camera::new(
        0.0,
        0.0,
        0.0,
        ORIGIN_X+WORLD_SIZE as f32 * VOXEL_SIZE/2.0,
        ORIGIN_Y+WORLD_SIZE as f32 * VOXEL_SIZE/3.0,
        ORIGIN_Z+WORLD_SIZE as f32 * VOXEL_SIZE/2.0
    );
    
    window.set_light(Light::StickToCamera);

    let mut world = World::new();
    let mut voxel_simulator = VoxelSimulator::new();
    let scene_generator = SceneGenerator::new(ORIGIN_X, ORIGIN_Y, ORIGIN_Z);
    let mut paused = false;
    let mut scene_map = SceneMap::new();
    let mut cursor = Cursor::new(WORLD_SIZE-1, WORLD_SIZE/2, WORLD_SIZE-1);
    let mut render_time;
    let mut generation_time = 0;
    let mut simulation_time = 0;
    let mut render_start_time = Instant::now();
    let mut generation_start_time;
    let mut simulation_start_time;
    let mut total_render_time = 0;
    while window.render_with_camera(camera.get_fp()) {
        render_time = render_start_time.elapsed().as_micros();
        total_render_time += render_time;

        if matches!(window.get_key(Key::W), Action::Press) {
            camera.move_z(render_time as f32 * CAMERA_MOVEMENT_SPEED)
        }
        if matches!(window.get_key(Key::S), Action::Press) {
            camera.move_z(-(render_time as f32) * CAMERA_MOVEMENT_SPEED)
        }
        if matches!(window.get_key(Key::A), Action::Press) {
            camera.move_x(render_time as f32 * CAMERA_MOVEMENT_SPEED)
        }
        if matches!(window.get_key(Key::D), Action::Press) {
            camera.move_x(-(render_time as f32) * CAMERA_MOVEMENT_SPEED)
        }
        if matches!(window.get_key(Key::Q), Action::Press) {
            camera.move_y(render_time as f32 * CAMERA_MOVEMENT_SPEED)
        }
        if matches!(window.get_key(Key::E), Action::Press) {
            camera.move_y(-(render_time as f32) * CAMERA_MOVEMENT_SPEED)
        }
        for mut e in window.events().iter() {
            match e.value {
                WindowEvent::Key(key, action, _modif) => {
                    e.inhibited = true;
                    cursor.input_key(key, action);
                    if matches!(key, Key::Space) && matches!(action, Action::Press) {
                        paused = !paused;
                    }
                    if matches!(key, Key::Escape) {
                        window.close();
                    }
                },
                WindowEvent::MouseButton(button, action, _modif) => {
                    e.inhibited = true;
                    match button {
                        kiss3d::event::MouseButton::Button1 => if matches!(action, kiss3d::event::Action::Press){
                            cursor.set_left_down(&mut world, &mut scene_map)
                        }else{
                            cursor.set_left_up(&mut world, &mut scene_map);
                        },
                        kiss3d::event::MouseButton::Button2 => if matches!(action, kiss3d::event::Action::Press){
                            cursor.set_right_down(&mut world, &mut scene_map)
                        }else{
                            cursor.set_right_up(&mut world, &mut scene_map);
                        },
                        _ => {},
                    }
                },
                WindowEvent::CursorPos(x, y, _modif) => {
                    e.inhibited = true;
                    cursor.input_move(1.0-x/window.width() as f64, 1.0-y/window.height() as f64, &mut world, &mut scene_map);
                },
                WindowEvent::Scroll(_a, b, _modif) => {
                    e.inhibited = true;
                    cursor.input_scroll(b, &mut world, &mut scene_map);
                },
                WindowEvent::FramebufferSize(x, y) => {
                    cursor.on_resize(x, y);
                },
                _ => {}
            }
        }
        cursor.update(&mut world, &mut scene_map);

        if total_render_time > TIME_BETWEEN_STEPS_US {
            if !paused {
                simulation_start_time = Instant::now();
                voxel_simulator.next_step(&mut world, &mut scene_map);
                simulation_time = simulation_start_time.elapsed().as_micros();
            }
            total_render_time -= TIME_BETWEEN_STEPS_US;
        }

        scene_generator.draw_border(&mut window);

        generation_start_time = Instant::now();
        let changed = scene_map.get_and_remove_changed(&mut window);
        scene_generator.generate_scene(&mut window, &world, &mut scene_map, changed);    
        generation_time = generation_start_time.elapsed().as_micros();

        print_time(render_time, generation_time, simulation_time);
        
        cursor.draw(&mut window, ORIGIN_X, ORIGIN_Y, ORIGIN_Z);
        cursor.draw_selected(&mut window);

        simulation_time = 0;
        generation_time = 0;
        render_start_time = Instant::now();
    }

    cursor.delete_nodes(&mut window);
}
