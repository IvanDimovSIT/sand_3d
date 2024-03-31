use std::{collections::HashMap, path::Path, rc::Rc};

use kiss3d::{context::Texture, resource::TextureManager};

use crate::model::VoxelMaterial;

pub struct TextureGenerator{
    texture_manager: TextureManager,
    textures: HashMap<u32, Rc<Texture>>
}
impl TextureGenerator {
    pub fn new() -> Self {
        let mut texture_manager = TextureManager::new();
        let mut textures: HashMap<u32, Rc<Texture>> = HashMap::new();
        
        let sand_texture = texture_manager.add(Path::new("textures/texture1.png"), "sand");
        textures.insert(VoxelMaterial::Sand.get_id(), sand_texture);

        let water_texture = texture_manager.add(Path::new("textures/texture1.png"), "sand");
        textures.insert(VoxelMaterial::Water.get_id(), water_texture);


        Self { texture_manager, textures}
    }

    pub fn get(&self, material: &VoxelMaterial) -> Rc<Texture> {
        self.textures.get(&material.get_id()).unwrap().clone()
    }
}