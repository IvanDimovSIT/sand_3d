#[derive(Clone, Copy, Debug)]
pub enum VoxelMaterial {
    Air,
    Sand,
    Water,
    Metal,
    Salt,
    SaltWater
}
impl VoxelMaterial {
    pub fn get_id(&self) -> u32 {
        match self {
            VoxelMaterial::Air => 0,
            VoxelMaterial::Sand => 1,
            VoxelMaterial::Water => 2,
            VoxelMaterial::Metal => 3,
            VoxelMaterial::Salt => 4,
            VoxelMaterial::SaltWater => 5,
        }
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
    voxels: [[[VoxelMaterial; WORLD_SIZE]; WORLD_SIZE]; WORLD_SIZE]
}
impl World {

    pub fn new() -> Self {
        let voxels = [[[VoxelMaterial::Air; WORLD_SIZE]; WORLD_SIZE]; WORLD_SIZE];

        Self { voxels }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> VoxelMaterial {
        self.voxels[z][y][x]
    }

    pub fn set(&mut self, material: VoxelMaterial, x: usize, y: usize, z: usize) {
        self.voxels[z][y][x] = material;
    }

    pub fn get_neighbours(&self, x: usize, y: usize, z: usize) -> VoxelNeighbours {
        let mut neighbours = VoxelNeighbours{top:false, bottom: false, left: false, right: false, front: false, back: false};
        if x > 1 && !matches!(self.voxels[z][y][x-1],VoxelMaterial::Air){
            neighbours.left = true;
        }
        if x+1 < WORLD_SIZE && !matches!(self.voxels[z][y][x+1],VoxelMaterial::Air){
            neighbours.right = true;
        }
        if y > 1 && !matches!(self.voxels[z][y-1][x],VoxelMaterial::Air){
            neighbours.bottom = true;
        }
        if y+1 < WORLD_SIZE && !matches!(self.voxels[z][y+1][x],VoxelMaterial::Air){
            neighbours.top = true;
        }

        if z > 1 && !matches!(self.voxels[z-1][y][x],VoxelMaterial::Air){
            neighbours.back = true;
        }
        if z+1 < WORLD_SIZE && !matches!(self.voxels[z+1][y][x],VoxelMaterial::Air){
            neighbours.front = true;
        }


        neighbours
    }
}