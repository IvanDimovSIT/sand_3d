use std::{cell::RefCell, f32::consts::PI, rc::Rc};

use kiss3d::{nalgebra::{OPoint, Point, Point2, Point3, Quaternion, Translation, Unit, UnitQuaternion, Vector3}, resource::{Material, Mesh}, scene::SceneNode, window::Window};

use crate::{model::{VoxelMaterial, VoxelNeighbours, World, WORLD_SIZE}, texture_generator::{self, TextureGenerator}, wire_cube::{self, WireCube}};

pub const VOXEL_SIZE: f32 = 10.0;
const BRIGHTNESS: f32 = 1.5;

pub struct SceneGenerator{
    texture_generator: TextureGenerator,
    origin: Translation<f32, 3>,
    left_rotation: Unit<Quaternion<f32>>,
    right_rotation: Unit<Quaternion<f32>>,
    up_rotation: Unit<Quaternion<f32>>,
    down_rotation: Unit<Quaternion<f32>>,
    back_rotation: Unit<Quaternion<f32>>,
    left_translation: Translation<f32, 3>,
    right_translation: Translation<f32, 3>,
    front_translation: Translation<f32, 3>,
    back_translation: Translation<f32, 3>,
    top_translation: Translation<f32, 3>,
    bottom_translation: Translation<f32, 3>,
    frame_border: WireCube,
    mesh: Rc<RefCell<Mesh>>
}
impl SceneGenerator {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let texture_generator = TextureGenerator::new();
        let origin = Translation::from(Vector3::new(x, y, z));
        let left_rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), -PI/2.0);
        let right_rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), PI/2.0);
        let up_rotation = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -PI/2.0);
        let down_rotation = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), PI/2.0);
        let back_rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), PI);
        let left_translation = Translation::from(Vector3::new(-VOXEL_SIZE/2.0, 0.0f32, 0.0f32));
        let right_translation = Translation::from(Vector3::new(VOXEL_SIZE/2.0, 0.0f32, 0.0f32));
        let front_translation = Translation::from(Vector3::new(0.0f32, 0.0f32, VOXEL_SIZE/2.0));
        let back_translation = Translation::from(Vector3::new(0.0f32, 0.0f32, -VOXEL_SIZE/2.0));
        let top_translation = Translation::from(Vector3::new(0.0f32, VOXEL_SIZE/2.0, 0.0f32));
        let bottom_translation = Translation::from(Vector3::new(0.0f32, -VOXEL_SIZE/2.0, 0.0f32));

        let frame_border = WireCube::new(x, y, z, WORLD_SIZE as f32 * VOXEL_SIZE, 0.0, 0.0, 0.0);
        
        let a = Point3::new(-VOXEL_SIZE/2.0, -VOXEL_SIZE/2.0, 0.0);
        let b = Point3::new(VOXEL_SIZE/2.0, -VOXEL_SIZE/2.0, 0.0);
        let c = Point3::new(VOXEL_SIZE/2.0, VOXEL_SIZE/2.0, 0.0);
        let d = Point3::new(-VOXEL_SIZE/2.0, VOXEL_SIZE/2.0, 0.0);
        let coords = vec![a, b, c, d];
        let faces = vec![Point3::new(0, 1, 2), Point3::new(2, 3, 0)];
        let uv_a = Point2::new(0.0, 0.0);
        let uv_b = Point2::new(1.0, 0.0);
        let uv_c = Point2::new(1.0, 1.0);
        let uv_d = Point2::new(0.0, 1.0);
        let uvs = vec![uv_a, uv_b, uv_c, uv_d];
        let mesh = Rc::new(RefCell::new(Mesh::new(coords, faces, None, Some(uvs), false)));

        Self {
            texture_generator,
            origin,
            left_rotation,
            right_rotation,
            up_rotation,
            down_rotation,
            back_rotation,
            left_translation,
            right_translation,
            front_translation,
            back_translation,
            top_translation,
            bottom_translation,
            frame_border,
            mesh
        }        
    }

    fn move_from_origin(&self, nodes: &mut Vec<SceneNode>, x: usize, y: usize, z: usize) {
        let translation = Translation::from(
            Vector3::new(x as f32 * VOXEL_SIZE, y as f32 * VOXEL_SIZE, z as f32 * VOXEL_SIZE)
        );

        for i in nodes {
            i.append_translation(&self.origin);
            i.append_translation(&translation);
        }
    }

    fn generate_sides(&self, nodes: &mut Vec<SceneNode>, window: &mut Window, neighbours: &VoxelNeighbours) {
        if !neighbours.top {
            let mut top = window.add_mesh(self.mesh.clone(), Vector3::new(1.0, 1.0, 1.0));
            top.append_rotation(&self.up_rotation);
            top.append_translation(&self.top_translation);
            nodes.push(top);
        }

        if !neighbours.bottom {
            let mut bottom = window.add_mesh(self.mesh.clone(), Vector3::new(1.0, 1.0, 1.0));
            bottom.append_rotation(&self.down_rotation);
            bottom.append_translation(&self.bottom_translation);
            nodes.push(bottom);
        }

        if !neighbours.left {
            let mut left = window.add_mesh(self.mesh.clone(), Vector3::new(1.0, 1.0, 1.0));
            left.append_rotation(&self.left_rotation);
            left.append_translation(&self.left_translation);
            nodes.push(left);
        }

        if !neighbours.right {
            let mut right = window.add_mesh(self.mesh.clone(), Vector3::new(1.0, 1.0, 1.0));
            right.append_rotation(&self.right_rotation);
            right.append_translation(&self.right_translation);
            nodes.push(right);
        }

        if !neighbours.front {
            let mut front = window.add_mesh(self.mesh.clone(), Vector3::new(1.0, 1.0, 1.0));
            front.append_translation(&self.front_translation);
            nodes.push(front);
        }

        if !neighbours.back {
            let mut back = window.add_mesh(self.mesh.clone(), Vector3::new(1.0, 1.0, 1.0));
            back.append_rotation(&self.back_rotation);
            back.append_translation(&self.back_translation);
            nodes.push(back);
        }

    }

    fn set_texutures(&self, nodes: &mut Vec<SceneNode>, material: &VoxelMaterial) {
        let texture = self.texture_generator.get(material);
        for i in nodes {
            i.set_texture(texture.clone());
            i.set_color(BRIGHTNESS, BRIGHTNESS, BRIGHTNESS);
        }
    }

    fn generate_nodes(&self, window: &mut Window, world: &World, x: usize, y: usize, z: usize) -> Vec<SceneNode> {
        let mut nodes: Vec<SceneNode> = vec![];
        let neighbours = world.get_neighbours(x, y, z);
        let material = world.get(x, y, z);
        match neighbours.get_neighbours_count() {
            6 => {return nodes;},
            0 => {
                let voxel = window.add_cube(VOXEL_SIZE, VOXEL_SIZE, VOXEL_SIZE);
                nodes.push(voxel);
            },
            _ => {
                let mut sides = vec![];
                self.generate_sides(&mut sides, window, &neighbours);
                nodes.append(&mut sides);
            },
        }
        self.move_from_origin(&mut nodes, x, y, z);
        self.set_texutures(&mut nodes, &material);
        
        nodes
    }

    pub fn generate_scene(&self, window: &mut Window, world: &World) -> Vec<SceneNode> {
        let mut nodes: Vec<SceneNode> = vec![];
        for y in (0..WORLD_SIZE).rev() {
            for z in 0..WORLD_SIZE {
                for x in 0..WORLD_SIZE {
                    if matches!(world.get(x, y, z), VoxelMaterial::Air) {
                        continue;
                    }

                    nodes.append(&mut self.generate_nodes(window, world, x, y, z));
                }
            }
        }

        nodes
    }

    pub fn draw_border(&self, window: &mut Window) {
        self.frame_border.draw(window);
    }
}

