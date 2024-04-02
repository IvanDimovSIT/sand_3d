use rand::{rngs::ThreadRng, thread_rng, Rng};
use rand::seq::SliceRandom;

use crate::material_reactions::MaterialReactions;
use crate::scene_map::SceneMap;
use crate::{material_properties::{MaterialProperties, MaterialType}, model::{VoxelMaterial, World, WORLD_SIZE}};

pub struct VoxelSimulator{
    rng: ThreadRng,
    down_neighbours: Vec<(isize, isize, isize)>,
    side_neighbours: Vec<(isize, isize, isize)>,
    up_neighbours: Vec<(isize, isize, isize)>,
    all_neighbours: Vec<(isize, isize, isize)>
}
impl VoxelSimulator {
    pub fn new() -> VoxelSimulator {
        let down_neighbours = vec![(-1,-1,1), (0,-1,1), (1,-1,1), (1,-1,0), (1,-1,-1), (0,-1,-1), (-1,-1,-1), (-1,-1,0)];
        let side_neighbours = vec![(-1,0,1), (0,0,1), (1,0,1), (1,0,0), (1,0,-1), (0,0,-1), (-1,0,-1), (-1,0,0)];
        let up_neighbours = vec![(-1,1,1), (0,1,1), (1,1,1), (1,1,0), (1,1,-1), (0,1,-1), (-1,1,-1), (-1,1,0), (0,1,0)];
        let mut all_neighbours = vec![(0,-1,0)];
        all_neighbours.append(&mut down_neighbours.clone());
        all_neighbours.append(&mut side_neighbours.clone());
        all_neighbours.append(&mut up_neighbours.clone());


        Self { rng: thread_rng(), down_neighbours, side_neighbours, up_neighbours, all_neighbours }
    }

    fn check_activity(&mut self, properties: &MaterialProperties) -> bool {
        properties.activity >= 1.0 || properties.activity > self.rng.gen_range(0.0..1.0)
    }

    fn swap(
        &mut self,
        world: &mut World,
        scene_map: &mut SceneMap,
        material: VoxelMaterial,
        properties: &MaterialProperties,
        x1: usize, y1: usize, z1: usize,
        x2: usize, y2: usize, z2: usize,
        mask: &mut [bool]) -> bool {
        if x2 >= WORLD_SIZE || y2 >= WORLD_SIZE || z2 >= WORLD_SIZE {
            return false;
        }
        if mask[World::coordiantes_to_index(x1, y1, z1)] || mask[World::coordiantes_to_index(x2, y2, z2)] {
            false;
        } 

        let other = world.get(x2, y2, z2);
        if matches!(other, VoxelMaterial::Air){
            world.set(material, x2, y2, z2);
            world.set(other, x1, y1, z1);
            mask[World::coordiantes_to_index(x2, y2, z2)] = true;

            scene_map.update(x1, y1, z1);
            scene_map.update(x2, y2, z2);
            true
        }else{
            let other_properties = MaterialProperties::new(&other);
            if matches!(other_properties.material_type, MaterialType::SOLID) {
                false
            }else if properties.weight > other_properties.weight {
                world.set(material, x2, y2, z2);
                world.set(other, x1, y1, z1);

                mask[World::coordiantes_to_index(x1, y1, z1)] = true;
                mask[World::coordiantes_to_index(x2, y2, z2)] = true;


                scene_map.update(x1, y1, z1);
                scene_map.update(x2, y2, z2);
                true
            }else{
                false
            }
        }
    }

    fn check_reaction(
        &mut self,
        world: &mut World,
        scene_map: &mut SceneMap,
        x: usize,
        y: usize,
        z: usize){
        let material = world.get(x, y, z);
        let mut reactions = MaterialReactions::new(&material);
        if reactions.is_empty() {
            return;
        }
        
        reactions.shuffle(&mut self.rng);
        let mut check_neighbours = self.all_neighbours.clone();
        check_neighbours.shuffle(&mut self.rng);
        for i in reactions {
            if !self.rng.gen_bool(i.probability as f64) {
                continue;
            }

            for j in &check_neighbours {
                let other_x = j.0 + x as isize;
                let other_y = j.1 + y as isize;
                let other_z = j.2 + z as isize;
                if other_x < 0 || other_x >= WORLD_SIZE as isize ||
                    other_y < 0 || other_y >= WORLD_SIZE as isize ||
                    other_z < 0 || other_z >= WORLD_SIZE as isize {
                    continue;
                }

                if world.get(other_x as usize, other_y as usize, other_z as usize).get_id() == i.other_material.get_id() {
                    world.set(i.first_product, x, y, z);
                    world.set(i.second_product, other_x as usize, other_y as usize, other_z as usize);
                    scene_map.update(x, y, z);
                    scene_map.update(other_x as usize, other_y as usize, other_z as usize);
                    return;
                }
            }
        }
    }

    fn simulate_liquid(
        &mut self,
        world: &mut World,
        scene_map: &mut SceneMap,
        material: VoxelMaterial,
        properties: MaterialProperties,
        x: usize,
        y: usize,
        z: usize,
        mask: &mut [bool]) {
        if y > 0 {
            if self.swap(world, scene_map, material, &properties, x, y, z, x, y-1, z, mask) {
                return;
            }
            
            let mut down_neighbours = self.down_neighbours.clone();
            down_neighbours.shuffle(&mut self.rng);

            for i in down_neighbours {
                let other_x = x as isize + i.0;
                let other_y = y as isize + i.1;
                let other_z = z as isize + i.2;
                if other_x < 0 || other_y < 0 || other_z < 0 {
                    continue;
                }

                if self.swap(world, scene_map, material, &properties, x, y, z, other_x as usize, other_y as usize, other_z as usize, mask) {
                    return;
                }
            }
        }

        if !self.check_activity(&properties){
            return;
        }

        let mut side_neighbours = self.side_neighbours.clone();
        side_neighbours.shuffle(&mut self.rng);

        for i in side_neighbours {
            let other_x = x as isize + i.0;
            let other_y = y as isize + i.1;
            let other_z = z as isize + i.2;
            if other_x < 0 || other_y < 0 || other_z < 0 {
                continue;
            }

            if self.swap(world, scene_map, material, &properties, x, y, z, other_x as usize, other_y as usize, other_z as usize, mask) {
                return;
            }
        }
    }

    fn simulate_powder(
        &mut self,
        world: &mut World,
        scene_map: &mut SceneMap,
        material: VoxelMaterial,
        properties: MaterialProperties,
        x: usize,
        y: usize,
        z: usize,
        mask: &mut [bool]) {
        if y <= 0 {
            return;
        }

        if self.swap(world, scene_map, material, &properties, x, y, z, x, y-1, z, mask) {
            return;
        }

        if !self.check_activity(&properties){
            return;
        }

        let mut down_neighbours = self.down_neighbours.clone();
        down_neighbours.shuffle(&mut self.rng);
    
        for i in down_neighbours {
            let other_x = x as isize + i.0;
            let other_y = y as isize + i.1;
            let other_z = z as isize + i.2;
            if other_x < 0 || other_y < 0 || other_z < 0 {
                continue;
            }
    
            if self.swap(world, scene_map, material, &properties, x, y, z, other_x as usize, other_y as usize, other_z as usize, mask) {
                return;
            }
        }
    }

    fn simulate_gas(
        &mut self,
        world: &mut World,
        scene_map: &mut SceneMap,
        material: VoxelMaterial,
        properties: MaterialProperties,
        x: usize,
        y: usize,
        z: usize,
        mask: &mut [bool]) {
        let mut neighbours = vec![];
        
        if self.rng.gen_bool(properties.activity as f64){
            neighbours = self.all_neighbours.clone();
        }else{
            let mut neighbours = self.up_neighbours.clone();
            neighbours.append(&mut self.side_neighbours.clone());
        }
        neighbours.shuffle(&mut self.rng);

        for i in neighbours {
            let other_x = x as isize + i.0;
            let other_y = y as isize + i.1;
            let other_z = z as isize + i.2;
            if other_x < 0 || other_y < 0 || other_z < 0 {
                continue;
            }
            if self.swap(world, scene_map, material, &properties, x, y, z, other_x as usize, other_y as usize, other_z as usize, mask) {
                return;
            }
        }
    }
    
    fn simulate_voxel(&mut self, world: &mut World, scene_map: &mut SceneMap, x: usize, y: usize, z: usize, mask: &mut [bool]) {
        let material = world.get(x, y, z);
        let material_properties = MaterialProperties::new(&material);
        match material_properties.material_type {
            MaterialType::SOLID => {},
            MaterialType::LIQUID => {self.simulate_liquid(world, scene_map, material, material_properties, x, y, z, mask)},
            MaterialType::POWDER => {self.simulate_powder(world, scene_map, material, material_properties, x, y, z, mask)},
            MaterialType::GAS => {self.simulate_gas(world, scene_map, material, material_properties, x, y, z, mask)},
        }
    }

    pub fn next_step(&mut self, world: &mut World, scene_map: &mut SceneMap) {
        let mut mask = [false; WORLD_SIZE*WORLD_SIZE*WORLD_SIZE];
        for y in 0..WORLD_SIZE {
            for z in 0..WORLD_SIZE {
                for x in 0..WORLD_SIZE {
                    if matches!(world.get(x, y, z), VoxelMaterial::Air) {
                        continue;
                    }
                    
                    self.simulate_voxel(world, scene_map, x, y, z, &mut mask);
                    self.check_reaction(world, scene_map,  x, y, z);
                }
            }
        }
    }
}