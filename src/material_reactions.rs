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
                    probability: 0.1
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
                    other_material: VoxelMaterial::Oil,
                    first_product: VoxelMaterial::Lava,
                    second_product: VoxelMaterial::Fire,
                    probability: 1.0
                },
                MaterialReactions{
                    other_material: VoxelMaterial::Ice,
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
                MaterialReactions{
                    other_material: VoxelMaterial::Wood,
                    first_product: VoxelMaterial::Lava,
                    second_product: VoxelMaterial::Fire,
                    probability: 0.9
                },
            ],
            VoxelMaterial::Fire => vec![
                MaterialReactions{ 
                    other_material: VoxelMaterial::Air,
                    first_product: VoxelMaterial::Air,
                    second_product: VoxelMaterial::Air,
                    probability: 0.03
                },
                MaterialReactions{ 
                    other_material: VoxelMaterial::Wood,
                    first_product: VoxelMaterial::Fire,
                    second_product: VoxelMaterial::Fire,
                    probability: 0.6
                },
                MaterialReactions{ 
                    other_material: VoxelMaterial::Oil,
                    first_product: VoxelMaterial::Fire,
                    second_product: VoxelMaterial::Fire,
                    probability: 1.0
                },
                MaterialReactions{ 
                    other_material: VoxelMaterial::Water,
                    first_product: VoxelMaterial::Air,
                    second_product: VoxelMaterial::Steam,
                    probability: 1.0
                },
                MaterialReactions{ 
                    other_material: VoxelMaterial::SaltWater,
                    first_product: VoxelMaterial::Air,
                    second_product: VoxelMaterial::Steam,
                    probability: 1.0
                },
                MaterialReactions{ 
                    other_material: VoxelMaterial::Ice,
                    first_product: VoxelMaterial::Air,
                    second_product: VoxelMaterial::Water,
                    probability: 1.0
                },
            ],
            VoxelMaterial::Ice => vec![
                MaterialReactions{ 
                    other_material: VoxelMaterial::Water,
                    first_product: VoxelMaterial::Ice,
                    second_product: VoxelMaterial::Ice,
                    probability: 0.02
                },
            ],
            _ => vec![],
        }
    }
}