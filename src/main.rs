extern crate kiss3d;

mod model;
mod scene_generator;
mod texture_generator;
mod wire_cube;

use std::path::Path;

use kiss3d::nalgebra::{Translation, UnitQuaternion, Vector3};
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::resource::{Material, MaterialManager, TextureManager};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use kiss3d::light::Light;
use model::World;
use scene_generator::SceneGenerator;

const FPS: u64 = 60;

// fn test_setup(window: &mut Window) {
//     let left_rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 3.14/2.0);
//         let up_rotation = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 3.14/2.0);
//         let left_translation = Translation::from(Vector3::new(-1.0/2.0, 0.0f32, 0.0f32));
//         let right_translation = Translation::from(Vector3::new(1.0/2.0, 0.0f32, 0.0f32));
//         let front_translation = Translation::from(Vector3::new(0.0f32, 0.0f32, 1.0/2.0));
//         let back_translation = Translation::from(Vector3::new(0.0f32, 0.0f32, -1.0/2.0));
//         let top_translation = Translation::from(Vector3::new(0.0f32, 1.0/2.0, 0.0f32));
//         let bottom_translation = Translation::from(Vector3::new(0.0f32, -1.0/2.0, 0.0f32));

//     let mut front = window.add_quad(1.0, 1.0, 1, 1);
//     front.set_color(1.0, 1.0, 1.0);
//     front.append_translation(&front_translation);

//     let mut back = window.add_quad(1.0, 1.0, 1, 1);
//     back.set_color(1.0, 1.0, 1.0);
//     back.append_translation(&back_translation);

//     let mut top = window.add_quad(1.0, 1.0, 1, 1);
//     top.set_color(1.0, 1.0, 1.0);
//     top.append_rotation(&up_rotation);
//     top.append_translation(&top_translation);

//     let mut bottom = window.add_quad(1.0, 1.0, 1, 1);
//     bottom.set_color(1.0, 1.0, 1.0);
//     bottom.append_rotation(&up_rotation);
//     bottom.append_translation(&bottom_translation);

//     let mut left = window.add_quad(1.0, 1.0, 1, 1);
//     left.set_color(1.0, 1.0, 1.0);
//     left.append_rotation(&left_rotation);
//     left.append_translation(&left_translation);

//     let mut right = window.add_quad(1.0, 1.0, 1, 1);
//     right.set_color(1.0, 1.0, 1.0);
//     right.append_rotation(&left_rotation);
//     right.append_translation(&right_translation);

//     let translation = Translation::from(
//         Vector3::new(1.0 * 1.0, 1.0 * 1.0, 1.0 * 1.0)
//     );

//     let translation2 = Translation::from(
//         Vector3::new(0.0, 4.0, 0.0)
//     );

//     top.append_translation(&translation);
//     bottom.append_translation(&translation);
//     left.append_translation(&translation);
//     right.append_translation(&translation);
//     front.append_translation(&translation);
//     back.append_translation(&translation);

//     top.append_translation(&translation2);
//     bottom.append_translation(&translation2);
//     left.append_translation(&translation2);
//     right.append_translation(&translation2);
//     front.append_translation(&translation2);
//     back.append_translation(&translation2);    
// }

fn main() {
    let mut window = Window::new("Sand 3D");
    window.set_framerate_limit(Some(FPS));
    window.set_background_color(0.8, 0.8, 0.9);
    //test_setup(&mut window);//TODO: remove
    //let mut texture_manager = TextureManager::new();
    //let tex1 = texture_manager.add(Path::new("textures/texture1.png"), "sand");
    //let mut c   = window.add_cube(1.0, 1.0, 1.0);
    //c.prepend_to_local_translation(&Translation::from(Vector3::new(0.0, 0.0, 3.0)));
    //c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);
    //let mut face = window.add_quad(1.0, 1.0, 1, 1);
    //face.prepend_to_local_translation(&Translation::from(Vector3::new(0.0f32, 0.0f32, 1.0f32)));
    //face.set_color(1.0, 1.0, 1.0);
    //face.set_texture(tex1.clone());

    //let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    //let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    //let t = Translation::from(Vector3::new(0.0f32, 0.0f32, 0.1f32));
    let mut world = World::new();
    world.set(model::VoxelMaterial::Sand, 2, 7, 2); // Initial setup
    world.set(model::VoxelMaterial::Sand, 3, 7, 2); // Initial setup
    world.set(model::VoxelMaterial::Sand, 2, 8, 2); // Initial setup

    let scene_generator = SceneGenerator::new(0.0, 0.0, 0.0);
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

        //face.prepend_to_local_rotation(&rot);
        //i += 1;
        //c.prepend_to_local_rotation(&rot);
    }
}
