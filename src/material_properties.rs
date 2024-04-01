use crate::model::VoxelMaterial;

pub enum MaterialType {
    LIQUID, POWDER, SOLID
}

pub struct MaterialProperties{
    pub material_type: MaterialType,
    pub weight: u32,
    pub activity: f32
}
impl MaterialProperties {
    pub fn new(material: &VoxelMaterial) -> Self{
        match material {
            VoxelMaterial::Air => {panic!("Air has no properties")},
            VoxelMaterial::Sand => MaterialProperties { material_type: MaterialType::POWDER, weight: 10, activity: 1.0 },
            VoxelMaterial::Water => MaterialProperties { material_type: MaterialType::LIQUID, weight: 5, activity: 1.0 },
            VoxelMaterial::Metal => MaterialProperties { material_type: MaterialType::SOLID, weight: 100, activity: 0.0 },
            VoxelMaterial::Salt => MaterialProperties { material_type: MaterialType::POWDER, weight: 10, activity: 1.0 },
            VoxelMaterial::SaltWater => MaterialProperties { material_type: MaterialType::LIQUID, weight: 6, activity: 1.0 },
        }
    }
}