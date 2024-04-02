use std::{collections::{HashMap, HashSet}, mem::replace};

use kiss3d::{scene::SceneNode, window::Window};

use crate::model::{World, WORLD_SIZE};


pub struct SceneMap{
    scene_nodes: HashMap<usize, Vec<SceneNode>>,
    to_update: HashSet<usize>
}
impl SceneMap{
    pub fn new() -> Self {
        let scene_nodes = HashMap::new();
        let mut to_update = HashSet::new();
        for y in 0..WORLD_SIZE {
            for z in 0..WORLD_SIZE {
                for x in 0..WORLD_SIZE {
                    to_update.insert(World::coordiantes_to_index(x, y, z));
                }    
            }
        }

        Self {
            scene_nodes,
            to_update
        }
    }

    pub fn add_mesh(&mut self, x: usize, y:usize, z:usize, nodes: Vec<SceneNode>) {
        self.scene_nodes.insert(World::coordiantes_to_index(x, y, z), nodes);
    }

    pub fn get_and_remove_changed(&mut self, window: &mut Window) -> HashSet<usize> {
        for i in &self.to_update {
            let n = self.scene_nodes.get_mut(i);
            if n.is_none() {
                continue;
            }
            let n = n.unwrap();
            for j in n {
                window.remove_node(j);
            }
        }
        replace(&mut self.to_update, HashSet::new()) 
    }

    pub fn update(&mut self, x: usize, y:usize, z:usize) {
        if x < WORLD_SIZE && y < WORLD_SIZE && z < WORLD_SIZE {
            self.to_update.insert(World::coordiantes_to_index(x, y, z));
        }
        if x >= 1 && x-1 < WORLD_SIZE && y < WORLD_SIZE && z < WORLD_SIZE {
            self.to_update.insert(World::coordiantes_to_index(x-1, y, z));
        }
        if x < WORLD_SIZE && y >= 1 && y-1 < WORLD_SIZE && z < WORLD_SIZE {
            self.to_update.insert(World::coordiantes_to_index(x, y-1, z));
        }
        if x < WORLD_SIZE && y < WORLD_SIZE && z >= 1 && z - 1 < WORLD_SIZE {
            self.to_update.insert(World::coordiantes_to_index(x, y, z-1));
        }
        if x+1 < WORLD_SIZE && y < WORLD_SIZE && z < WORLD_SIZE {
            self.to_update.insert(World::coordiantes_to_index(x+1, y, z));
        }
        if x < WORLD_SIZE && y+1 < WORLD_SIZE && z < WORLD_SIZE {
            self.to_update.insert(World::coordiantes_to_index(x, y+1, z));
        }
        if  x < WORLD_SIZE && y < WORLD_SIZE && z + 1 < WORLD_SIZE {
            self.to_update.insert(World::coordiantes_to_index(x, y, z+1));
        }
    }
}