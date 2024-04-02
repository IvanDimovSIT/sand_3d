use kiss3d::{event::MouseButton, window::Window};

use crate::{model::{VoxelMaterial, World, WORLD_SIZE}, scene_generator::VOXEL_SIZE, scene_map::SceneMap, wire_cube::WireCube};

pub struct Cursor{
    x: usize,
    y: usize,
    z: usize,
    material: VoxelMaterial
}
impl Cursor{
    pub fn new(x: usize, y: usize, z: usize, material: VoxelMaterial) -> Self {
        Self { x, y, z, material }
    }

    pub fn input_move(&mut self, x: f64, y: f64) {
        self.x = (x * WORLD_SIZE as f64).floor() as usize;
        self.z = (y * WORLD_SIZE as f64).floor() as usize;
    }

    pub fn input_scroll(&mut self, amount: f64) {
        if amount < 0.0 && self.y > 0{
            self.y -= 1;
        }else if amount > 0.0 && self.y + 1 < WORLD_SIZE {
            self.y += 1;
        }
    }
    
    pub fn input_click(&self, world: &mut World, scene_map: &mut SceneMap, mouse: MouseButton) {
        if self.x >= WORLD_SIZE || self.y >= WORLD_SIZE || self.z >= WORLD_SIZE {
            return;
        }
        match mouse {
            MouseButton::Button1 => {
                world.set(self.material, self.x, self.y, self.z);
            },
            MouseButton::Button2 => {
                world.set(VoxelMaterial::Air, self.x, self.y, self.z);
            },
            _ => {return;}
        }
        scene_map.update(self.x, self.y, self.z);
    }

    pub fn draw(&self, window: &mut Window, offset_x: f32, offset_y: f32, offset_z: f32) {
        let wire_cube = WireCube::new(
            offset_x + self.x as f32 * VOXEL_SIZE - VOXEL_SIZE/2.0,
            offset_y + self.y as f32 * VOXEL_SIZE - VOXEL_SIZE/2.0,
            offset_z + self.z as f32 * VOXEL_SIZE - VOXEL_SIZE/2.0,
            VOXEL_SIZE,
            1.0, 0.1, 0.1
        );

        wire_cube.draw(window);
    }
}
