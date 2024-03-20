mod rendering;

use std::f32::consts::PI;
use std::mem::take;

use raylib::prelude::*;
use raylib::core::drawing::*;
use rendering::CubeFaceGenerator;


const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

const CAMERA_FOV: f32 = 60.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Sand 3D")
        .build();
    
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
            //let plane_mesh = Mesh::gen_mesh_plane(&thread, 1.0, 1.0, 1, 1);
            //let plane_mesh_2 = Mesh::gen_mesh_plane(&thread, 1.0, 1.0, 1, 1);
            //context_3d.draw_mesh(plane_mesh, context_3d.load_material_default(&thread) , Matrix::identity());

            //let mut matrix = Matrix::rotate_z(-PI/2.0);
            //matrix *= Matrix::translate(1.0, 0.0, 0.0);
            

            let cube_position = Vector3{x:0.0, y:1.0, z:1.0};
            let (gen_plane_mesh, gen_matrix) = cube_face_generator.generate_front(&thread, cube_position);
            context_3d.draw_mesh(gen_plane_mesh, context_3d.load_material_default(&thread) , gen_matrix);

            let (gen_plane_mesh, gen_matrix) = cube_face_generator.generate_down(&thread, cube_position);
            context_3d.draw_mesh(gen_plane_mesh, context_3d.load_material_default(&thread) , gen_matrix);

            let (gen_plane_mesh, gen_matrix) = cube_face_generator.generate_top(&thread, cube_position);
            context_3d.draw_mesh(gen_plane_mesh, context_3d.load_material_default(&thread) , gen_matrix);

            let (gen_plane_mesh, gen_matrix) = cube_face_generator.generate_back(&thread, cube_position);
            context_3d.draw_mesh(gen_plane_mesh, context_3d.load_material_default(&thread) , gen_matrix);
            
            let (gen_plane_mesh, gen_matrix) = cube_face_generator.generate_left(&thread, cube_position);
            context_3d.draw_mesh(gen_plane_mesh, context_3d.load_material_default(&thread) , gen_matrix);
            
            let (gen_plane_mesh, gen_matrix) = cube_face_generator.generate_right(&thread, cube_position);
            context_3d.draw_mesh(gen_plane_mesh, context_3d.load_material_default(&thread) , gen_matrix);


            //context_3d.draw_plane(Vector3::zero(), Vector2::one(), Color::RED);
        }
        d.draw_text("Sand 3D", 12, 12, 20, Color::BLACK);
        d.draw_fps(WIDTH-90, 5);
    }
}