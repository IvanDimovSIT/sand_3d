mod rendering;

use std::f32::consts::PI;
use std::mem::take;

use raylib::prelude::*;
use raylib::core::drawing::*;
use rendering::{CubeFaceGenerator, CubeRenderer};


const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

const CAMERA_FOV: f32 = 60.0;
const FPS: u32 = 60;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Sand 3D")
        .build();
    rl.set_target_fps(FPS);
    let shader1 = rl.load_shader(&thread, Some("shaders/shader1.vs"), Some("shares/shader2.fs")).unwrap();
    let shaders = vec![shader1];
    
    let texture1 = rl.load_texture(&thread, "textures/texture1.png").unwrap();

    let mut material1 = rl.load_material_default(&thread);
    material1.set_material_texture(MaterialMapIndex::MATERIAL_MAP_ALBEDO, texture1);

    let materials = vec![material1];
    
    let light = Light::default_directional(Vector3::new(-1.0, -1.0, -1.0), Color::WHITE);

    rl.set_light_direction(light);
    let mut camera = Camera3D::perspective(
        Vector3{x: -10.0, y:5.0, z: 2.0},
        Vector3::zero(),
        Vector3::up(),
        CAMERA_FOV
    );

    let cube_face_generator = CubeFaceGenerator::new();

    while !rl.window_should_close() {
        let delta = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);
        camera.position.x += delta;
        camera.position.z -= delta;
        if camera.position.x > - 1.0 {
            camera.position.x = -10.0;
        }

        if camera.position.y < -2.0 {
            camera.position.x = 5.0;
        }
        
        d.clear_background(Color::DARKGRAY);
        { // Draw 3D
            let mut context_3d = d.begin_mode3D(camera.clone());
            context_3d.draw_grid(100, 1.0);

            let cube_position = Vector3{x:0.0, y:1.0, z:1.0};
            let mut faces: Vec<Matrix> = vec![];
            faces.push(cube_face_generator.generate_front(cube_position));
            faces.push(cube_face_generator.generate_back(cube_position));
            faces.push(cube_face_generator.generate_top(cube_position));
            faces.push(cube_face_generator.generate_down(cube_position));
            faces.push(cube_face_generator.generate_left(cube_position));
            faces.push(cube_face_generator.generate_right(cube_position));


            
            CubeRenderer::render(&mut context_3d, &thread, &faces, &shaders, &materials, Vector3 { x: 0.0, y: 1.0, z: 1.0 });

        }
        d.draw_text("Sand 3D", 12, 12, 20, Color::BLACK);
        d.draw_fps(WIDTH-90, 5);
    }
}