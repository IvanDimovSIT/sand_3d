use std::any::Any;

use kiss3d::resource::Material;
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::{material_properties::{MaterialProperties, MaterialType}, model::{VoxelMaterial, World, WORLD_SIZE}};

enum Direction{
    Front,
    FrontRight,
    Right,
    BackRight,
    Back,
    BackLeft,
    Left,
    FrontLeft
}

pub struct VoxelSimulator{
    rng: ThreadRng
}
impl VoxelSimulator {
    pub fn new() -> VoxelSimulator {
        Self { rng: thread_rng() }
    }

    fn check_activity(&mut self, properties: &MaterialProperties) -> bool {
        properties.activity >= 1.0 || properties.activity > self.rng.gen_range(0.0..1.0)
    }

    fn get_direction(&mut self) -> Direction {
        match self.rng.gen_range(0..=7) {
            0 => Direction::Front,
            1 => Direction::FrontRight,
            2 => Direction::Right,
            3 => Direction::BackRight,
            4 => Direction::Back,
            5 => Direction::BackLeft,
            6 => Direction::Left,
            7 => Direction::FrontLeft,
            _ => unreachable!(),
        }
    }

    fn swap(
        &mut self,
        world: &mut World,
        material: VoxelMaterial,
        properties: &MaterialProperties,
        x1: usize, y1: usize, z1: usize,
        x2: usize, y2: usize, z2: usize) -> bool {
        if x2 >= WORLD_SIZE || y2 >= WORLD_SIZE || z2 >= WORLD_SIZE {
            return false;
        } 

        let other = world.get(x2, y2, z2);
        if matches!(other, VoxelMaterial::Air){
            world.set(material, x2, y2, z2);
            world.set(other, x1, y1, z1);
            
            true
        }else{
            let other_properties = MaterialProperties::new(&other);
            if properties.weight > other_properties.weight {
                world.set(material, x2, y2, z2);
                world.set(other, x1, y1, z1);

                true
            }else{
                false
            }
        }
    }

    fn simulate_liquid(
        &mut self,
        world: &mut World,
        has_changed: &mut bool,
        material: VoxelMaterial,
        properties: MaterialProperties,
        x: usize,
        y: usize,
        z: usize) {
        if y > 0 {
            if self.swap(world, material, &properties, x, y, z, x, y-1, z) {
                *has_changed = true;
                return;
            }
            if self.swap(world, material, &properties, x, y, z, x+1, y-1, z) {
                *has_changed = true;
                return;
            }
            if self.swap(world, material, &properties, x, y, z, x, y-1, z+1) {
                *has_changed = true;
                return;
            }
            if self.swap(world, material, &properties, x, y, z, x+1, y-1, z+1) {
                *has_changed = true;
                return;
            }
            if x > 0 {
                if self.swap(world, material, &properties, x, y, z, x-1, y-1, z) {
                    *has_changed = true;
                    return;
                }
            }
            if z > 0 {
                if self.swap(world, material, &properties, x, y, z, x, y-1, z-1) {
                    *has_changed = true;
                    return;
                }
            }
        }

        if !self.check_activity(&properties){
            return;
        }
        match self.get_direction() {
            Direction::Front => if self.swap(world, material, &properties, x, y, z, x, y, z+1) {
                *has_changed = true;
            },
            Direction::FrontRight => if self.swap(world, material, &properties, x, y, z, x+1, y, z+1) {
                *has_changed = true;
            },
            Direction::Right => if self.swap(world, material, &properties, x, y, z, x+1, y, z) {
                *has_changed = true;
            },
            Direction::BackRight => if z > 0 && self.swap(world, material, &properties, x, y, z, x+1, y, z-1) {
                *has_changed = true;
            },
            Direction::Back => if z > 0 && self.swap(world, material, &properties, x, y, z, x, y, z-1) {
                *has_changed = true;
            },
            Direction::BackLeft => if x > 0 && z > 0 && self.swap(world, material, &properties, x, y, z, x-1, y, z-1) {
                *has_changed = true;
            },
            Direction::Left => if x > 0 && self.swap(world, material, &properties, x, y, z, x-1, y, z) {
                *has_changed = true;
            },
            Direction::FrontLeft => if x > 0 && self.swap(world, material, &properties, x, y, z, x-1, y, z+1) {
                *has_changed = true;
            },
        }
    }

    fn simulate_powder(
        &mut self,
        world: &mut World,
        has_changed: &mut bool,
        material: VoxelMaterial,
        properties: MaterialProperties,
        x: usize,
        y: usize,
        z: usize) {
        if y > 0 {
            if self.swap(world, material, &properties, x, y, z, x, y-1, z) {
                *has_changed = true;
                return;
            }
            
            if !self.check_activity(&properties) {
                return;
            }

            if self.swap(world, material, &properties, x, y, z, x+1, y-1, z) {
                *has_changed = true;
                return;
            }
            if self.swap(world, material, &properties, x, y, z, x, y-1, z+1) {
                *has_changed = true;
                return;
            }
            if self.swap(world, material, &properties, x, y, z, x+1, y-1, z+1) {
                *has_changed = true;
                return;
            }
            if x > 0 {
                if self.swap(world, material, &properties, x, y, z, x-1, y-1, z) {
                    *has_changed = true;
                    return;
                }
            }
            if z > 0 {
                if self.swap(world, material, &properties, x, y, z, x, y-1, z-1) {
                    *has_changed = true;
                    return;
                }
            }
        }
    }
    
    fn simulate_voxel(&mut self, world: &mut World, has_changed: &mut bool, x: usize, y: usize, z: usize) {
        let material = world.get(x, y, z);
        let material_properties = MaterialProperties::new(&material);
        match material_properties.material_type {
            MaterialType::SOLID => {},
            MaterialType::LIQUID => {self.simulate_liquid(world, has_changed, material, material_properties, x, y, z)},
            MaterialType::POWDER => {self.simulate_powder(world, has_changed, material, material_properties, x, y, z)},
        }
    }

    pub fn next_step(&mut self, world: &mut World) -> bool {
        let mut has_changed = false;
        for y in 0..WORLD_SIZE {
            for z in 0..WORLD_SIZE {
                for x in 0..WORLD_SIZE {
                    if matches!(world.get(x, y, z), VoxelMaterial::Air) {
                        continue;
                    }
                    
                    self.simulate_voxel(world, &mut has_changed, x, y, z);
                }
            }
        }

        has_changed
    }
}