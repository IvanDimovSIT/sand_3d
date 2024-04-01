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
        
        let sand_texture = texture_manager.add(Path::new("textures/sand.png"), "sand");
        textures.insert(VoxelMaterial::Sand.get_id(), sand_texture);

        let water_texture = texture_manager.add(Path::new("textures/water.png"), "water");
        textures.insert(VoxelMaterial::Water.get_id(), water_texture);

        let metal_texture = texture_manager.add(Path::new("textures/metal.png"), "metal");
        textures.insert(VoxelMaterial::Metal.get_id(), metal_texture);

        let salt_texture = texture_manager.add(Path::new("textures/salt.png"), "salt");
        textures.insert(VoxelMaterial::Salt.get_id(), salt_texture);

        let salt_water_texture = texture_manager.add(Path::new("textures/salt_water.png"), "salt_water");
        textures.insert(VoxelMaterial::SaltWater.get_id(), salt_water_texture);

        let steam_texture = texture_manager.add(Path::new("textures/steam.png"), "steam");
        textures.insert(VoxelMaterial::Steam.get_id(), steam_texture);

        let stone_texture = texture_manager.add(Path::new("textures/stone.png"), "stone");
        textures.insert(VoxelMaterial::Stone.get_id(), stone_texture);

        let lava_texture = texture_manager.add(Path::new("textures/lava.png"), "lava");
        textures.insert(VoxelMaterial::Lava.get_id(), lava_texture);


        Self { texture_manager, textures}
    }

    pub fn get(&self, material: &VoxelMaterial) -> Rc<Texture> {
        self.textures.get(&material.get_id()).unwrap().clone()
    }
}