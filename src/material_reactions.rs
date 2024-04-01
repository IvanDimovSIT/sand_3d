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
                    first_product: VoxelMaterial::SaltWater,
                    second_product: VoxelMaterial::SaltWater,
                    probability: 0.2
                }
            ],
            _ => vec![],
        }
    }
}