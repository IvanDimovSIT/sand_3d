use crate::model::VoxelMaterial;


pub struct MaterialReactions{
    pub other_material: VoxelMaterial,
    pub first_product: VoxelMaterial,
    pub second_product: VoxelMaterial,
    pub probability: f32
}
impl MaterialReactions {
    pub fn new(material: &VoxelMaterial) -> Vec<MaterialReactions> {
        match material {
            VoxelMaterial::Salt => vec![
                MaterialReactions{
                    other_material: VoxelMaterial::Water,
                    first_product: VoxelMaterial::Air,
                    second_product: VoxelMaterial::SaltWater,
                    probability: 0.2
                }
            ],
            VoxelMaterial::Steam => vec![
                MaterialReactions{
                    other_material: VoxelMaterial::Air,
                    first_product: VoxelMaterial::Water,
                    second_product: VoxelMaterial::Air,
                    probability: 0.01
                }
            ],
            VoxelMaterial::Lava => vec![
                MaterialReactions{
                    other_material: VoxelMaterial::Water,
                    first_product: VoxelMaterial::Stone,
                    second_product: VoxelMaterial::Steam,
                    probability: 1.0
                },
                MaterialReactions{
                    other_material: VoxelMaterial::SaltWater,
                    first_product: VoxelMaterial::Stone,
                    second_product: VoxelMaterial::Steam,
                    probability: 1.0
                },
            ],
            _ => vec![],
        }
    }
}