use std::{collections::VecDeque, mem::take, rc::Rc};

use kiss3d::{event::{Action, Key, MouseButton}, nalgebra::{Point2, Point3, Translation2}, ncollide3d::bounding_volume, scene::PlanarSceneNode, text::Font, window::Window};

use crate::{model::{VoxelMaterial, World, WORLD_SIZE}, scene_generator::VOXEL_SIZE, scene_map::SceneMap, wire_cube::WireCube};

const FONT_SIZE: f32 = 46.0;
const SELECTED_BG_WIDTH: f32 = 400.0;
const SELECTED_BG_HEIGHT: f32 = 50.0;
const LINES_WIDTH: f32 = 2.0;

pub struct Cursor{
    x: usize,
    y: usize,
    z: usize,
    materials: VecDeque<VoxelMaterial>,
    font: Rc<Font>,
    left_down: bool,
    right_down: bool,
    selected_bg: Option<PlanarSceneNode>
}
impl Cursor{
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        let materials = VecDeque::from([
            VoxelMaterial::Sand,
            VoxelMaterial::Water,
            VoxelMaterial::Steam,
            VoxelMaterial::Salt,
            VoxelMaterial::SaltWater,
            VoxelMaterial::Lava,
            VoxelMaterial::Stone,
            VoxelMaterial::Metal,
            VoxelMaterial::Wood,
            VoxelMaterial::Fire,
            VoxelMaterial::Oil,
            VoxelMaterial::Ice,  
        ]);
        let font = Font::default();
        
        Self { x, y, z, materials, font, selected_bg: None, left_down: false, right_down: false }
    }

    pub fn input_key(&mut self, key: Key, action: Action) {
        if !matches!(action, Action::Press) {
            return;        
        }
        match key {
            Key::R | Key::Key2 => {
                let material = self.materials.pop_back().unwrap();
                self.materials.push_front(material)
            },
            Key::T | Key::Key1 => {
                let material = self.materials.pop_front().unwrap();
                self.materials.push_back(material)
            },
            _ => {},
        }
    }

    pub fn set_left_down(&mut self , world: &mut World, scene_map: &mut SceneMap) {
        self.left_down = true;
        self.update(world, scene_map);
    }

    pub fn set_right_down(&mut self , world: &mut World, scene_map: &mut SceneMap) {
        self.right_down = true;
        self.update(world, scene_map);
    }

    pub fn set_left_up(&mut self , world: &mut World, scene_map: &mut SceneMap) {
        self.left_down = false;
        self.update(world, scene_map);
    }

    pub fn set_right_up(&mut self , world: &mut World, scene_map: &mut SceneMap) {
        self.right_down = false;
        self.update(world, scene_map);
    }

    pub fn input_move(&mut self, x: f64, y: f64, world: &mut World, scene_map: &mut SceneMap) {
        self.x = (x * WORLD_SIZE as f64).floor() as usize;
        self.z = (y * WORLD_SIZE as f64).floor() as usize;
        self.update(world, scene_map);
    }

    pub fn input_scroll(&mut self, amount: f64, world: &mut World, scene_map: &mut SceneMap) {
        if amount < 0.0 && self.y > 0{
            self.y -= 1;
        }else if amount > 0.0 && self.y + 1 < WORLD_SIZE {
            self.y += 1;
        }else{
            return;
        }
        self.update(world, scene_map);
    }
    
    pub fn update(&self, world: &mut World, scene_map: &mut SceneMap) {
        if self.x >= WORLD_SIZE || self.y >= WORLD_SIZE || self.z >= WORLD_SIZE {
            return;
        }
        if self.left_down {
            world.set(*self.materials.front().unwrap(), self.x, self.y, self.z);    
        }else if self.right_down {
            world.set(VoxelMaterial::Air, self.x, self.y, self.z);       
        }else{
            return;
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

    pub fn on_resize(&mut self, x: u32, y: u32) {
        if self.selected_bg.is_none() {
            return;
        }
        let mut bg = take(&mut self.selected_bg).unwrap();
        let translation = Translation2::new(
            LINES_WIDTH-(x as f32)/2.0,
            LINES_WIDTH+(y as f32)/2.0
        );
        bg.set_local_translation(translation);
        self.selected_bg = Some(bg);
    }

    pub fn draw_selected(&mut self, window: &mut Window) {
        if self.selected_bg.is_none() {
            let mut bg = window.add_rectangle(SELECTED_BG_WIDTH, SELECTED_BG_HEIGHT);
            let translation = Translation2::new(
                LINES_WIDTH-(window.width() as f32)/2.0,
                LINES_WIDTH+(window.height() as f32)/2.0
            );
            bg.set_local_translation(translation);
            bg.set_color(1.0, 1.0, 1.0);
            bg.set_lines_width(LINES_WIDTH);
            bg.set_lines_color(Some(Point3::new(0.0, 0.0, 0.0)));
            self.selected_bg = Some(bg);
        }
        window.draw_text(
            &format!("Selected:{}", self.materials.front().unwrap()),
            &Point2::new(0.0, 0.0),
            FONT_SIZE,
            &self.font,
            &Point3::new(0.0, 0.0, 0.0)
        );
    }

    pub fn delete_nodes(&mut self, window: &mut Window) {
        if let Some(mut bg) =  take(&mut self.selected_bg) {
            window.remove_planar_node(&mut bg);
        }
    }
}
