use std::f32::consts::PI;

use raylib::prelude::*;
use raylib::core::drawing::*;

const CUBE_SIZE: f32 = 1.0;

pub struct CubeFaceGenerator{
    top: Matrix,
    down: Matrix,
    left: Matrix,
    right: Matrix,
    front: Matrix,
    back: Matrix,
}
impl CubeFaceGenerator{
    pub fn new() -> Self {
        let top = Matrix::translate(0.0, CUBE_SIZE/2.0, 0.0);

        let down = Matrix::rotate_z(-PI)* Matrix::translate(0.0, -CUBE_SIZE/2.0, 0.0);
        
        let left = Matrix::rotate_x(PI/2.0) * Matrix::translate(0.0, 0.0, -CUBE_SIZE/2.0);
        
        let right = Matrix::rotate_x(-PI/2.0) * Matrix::translate(0.0, 0.0, CUBE_SIZE/2.0);

        let front = Matrix::rotate_z(-PI/2.0) * Matrix::translate(-CUBE_SIZE/2.0, 0.0, 0.0);

        let back = Matrix::rotate_z(PI/2.0) * Matrix::translate(CUBE_SIZE/2.0, 0.0, 0.0);


        Self { top, down, left, right, front, back }
    }

    fn generate_mesh(thread: &RaylibThread) -> Mesh {
        Mesh::gen_mesh_plane(&thread, CUBE_SIZE, CUBE_SIZE, 1, 1)
    }

    pub fn generate_front(&self, thread: &RaylibThread, position: Vector3) -> (Mesh, Matrix) {
        let matrix = self.front.clone() * Matrix::translate(position.x, position.y, position.z);
        (Self::generate_mesh(thread), matrix)
    }

    pub fn generate_back(&self, thread: &RaylibThread, position: Vector3) -> (Mesh, Matrix) {
        let matrix = self.back.clone() * Matrix::translate(position.x, position.y, position.z);
        (Self::generate_mesh(thread), matrix)
    }

    pub fn generate_down(&self, thread: &RaylibThread, position: Vector3) -> (Mesh, Matrix) {
        let matrix = self.down.clone() * Matrix::translate(position.x, position.y, position.z);
        (Self::generate_mesh(thread), matrix)
    }

    pub fn generate_top(&self, thread: &RaylibThread, position: Vector3) -> (Mesh, Matrix) {
        let matrix = self.top.clone() * Matrix::translate(position.x, position.y, position.z);
        (Self::generate_mesh(thread), matrix)
    }

    pub fn generate_left(&self, thread: &RaylibThread, position: Vector3) -> (Mesh, Matrix) {
        let matrix = self.left.clone() * Matrix::translate(position.x, position.y, position.z);
        (Self::generate_mesh(thread), matrix)
    }

    pub fn generate_right(&self, thread: &RaylibThread, position: Vector3) -> (Mesh, Matrix) {
        let matrix = self.right.clone() * Matrix::translate(position.x, position.y, position.z);
        (Self::generate_mesh(thread), matrix)
    }
}