use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum VoxelMaterial {
    Air,
    Sand,
    Water,
    Metal,
    Salt,
    SaltWater,
    Steam,
    Lava,
    Stone,
    Wood,
    Fire,
    Oil,
    Ice
}
impl VoxelMaterial {
    pub fn get_id(&self) -> u32 {
        match self {
            Self::Air => 0,
            Self::Sand => 1,
            Self::Water => 2,
            Self::Metal => 3,
            Self::Salt => 4,
            Self::SaltWater => 5,
            Self::Steam => 6,
            Self::Stone => 7,
            Self::Lava => 8,
            Self::Wood => 9,
            Self::Fire => 10,
            Self::Oil => 11,
            Self::Ice => 12,
        }
    }
}
impl Display for VoxelMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Air => "Air",
            Self::Sand => "Sand",
            Self::Water => "Water",
            Self::Metal => "Metal",
            Self::Salt =>  "Salt",
            Self::SaltWater => "Salt Water",
            Self::Steam => "Steam",
            Self::Lava => "Lava",
            Self::Stone => "Stone",
            Self::Wood => "Wood",
            Self::Fire => "Fire",
            Self::Oil => "Oil",
            Self::Ice => "Ice",
        })
    }
}

pub const WORLD_SIZE: usize = 30;

pub struct VoxelNeighbours{
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub front: bool,
    pub back: bool,
}
impl VoxelNeighbours {
    pub fn get_neighbours_count(&self) -> u32 {
        self.top as u32 +
        self.bottom as u32 +
        self.left as u32 +
        self.right as u32 +
        self.front as u32 +
        self.back as u32
    }
}

pub struct World{
    voxels: [VoxelMaterial; WORLD_SIZE*WORLD_SIZE*WORLD_SIZE]
}
impl World {

    pub fn new() -> Self {
        let voxels = [VoxelMaterial::Air; WORLD_SIZE*WORLD_SIZE*WORLD_SIZE];

        Self { voxels }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> VoxelMaterial {
        self.voxels[Self::coordiantes_to_index(x, y, z)]
    }

    pub fn get_index(&self, index: usize) -> VoxelMaterial {
        self.voxels[index]
    }

    pub fn coordiantes_to_index(x: usize, y: usize, z: usize) -> usize {
        x + z*WORLD_SIZE + y*WORLD_SIZE*WORLD_SIZE
    }

    pub fn index_to_coordinates(index: usize) -> (usize, usize, usize) {
        let y = index / (WORLD_SIZE * WORLD_SIZE);
        let r = index % (WORLD_SIZE * WORLD_SIZE);
        let z = r / WORLD_SIZE;
        let x = r % WORLD_SIZE;
        (x, y, z)
    }

    pub fn set(&mut self, material: VoxelMaterial, x: usize, y: usize, z: usize) {
        self.voxels[Self::coordiantes_to_index(x, y, z)] = material;
    }

    pub fn get_neighbours(&self, x: usize, y: usize, z: usize) -> VoxelNeighbours {
        let mut neighbours = VoxelNeighbours{top:false, bottom: false, left: false, right: false, front: false, back: false};
        if x > 1 && !matches!(self.voxels[Self::coordiantes_to_index(x-1, y, z)],VoxelMaterial::Air){
            neighbours.left = true;
        }
        if x+1 < WORLD_SIZE && !matches!(self.voxels[Self::coordiantes_to_index(x+1, y, z)],VoxelMaterial::Air){
            neighbours.right = true;
        }
        if y > 1 && !matches!(self.voxels[Self::coordiantes_to_index(x, y-1, z)],VoxelMaterial::Air){
            neighbours.bottom = true;
        }
        if y+1 < WORLD_SIZE && !matches!(self.voxels[Self::coordiantes_to_index(x, y+1, z)],VoxelMaterial::Air){
            neighbours.top = true;
        }

        if z > 1 && !matches!(self.voxels[Self::coordiantes_to_index(x, y, z-1)],VoxelMaterial::Air){
            neighbours.back = true;
        }
        if z+1 < WORLD_SIZE && !matches!(self.voxels[Self::coordiantes_to_index(x, y, z+1)],VoxelMaterial::Air){
            neighbours.front = true;
        }


        neighbours
    }
}

#[cfg(test)]
mod tests{
    use super::{World, WORLD_SIZE};

    #[test]
    fn test() {
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                for z in 0..WORLD_SIZE {
                    let index = World::coordiantes_to_index(x, y, z);
                    let (x1, y1, z1) = World::index_to_coordinates(index);
                    assert_eq!(x, x1);
                    assert_eq!(y, y1);
                    assert_eq!(z, z1);
                }
            }
        }
    }
}