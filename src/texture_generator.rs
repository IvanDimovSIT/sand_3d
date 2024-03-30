use std::{collections::HashMap, path::Path, rc::Rc};

use kiss3d::{context::Texture, resource::TextureManager};

use crate::model::VoxelMaterial;

pub struct TextureGenerator{
    texture_manager: TextureManager,
    textures: Vec<Rc<Texture>>
}
impl TextureGenerator {
    pub fn new() -> Self {
        let mut texture_manager = TextureManager::new();
        let mut textures: Vec<Rc<Texture>> = vec![];
        
        let sand_texture = texture_manager.add(Path::new("textures/texture1.png"), "sand");
        textures.push(sand_texture);

        let water_texture = texture_manager.add(Path::new("textures/texture1.png"), "sand");
        textures.push(water_texture);

        Self { texture_manager, textures}
    }

    pub fn get(&self, material: &VoxelMaterial) -> Rc<Texture> {
        match material {
            VoxelMaterial::Sand => self.textures.get(0).unwrap().clone(),
            VoxelMaterial::Water => self.textures.get(1).unwrap().clone(), 
            VoxelMaterial::Air => {panic!("Air has no texture")},
            _ => {panic!("Material not implemented")},
        }
    }
}