extern crate nalgebra;

mod game_objects;


pub struct Simulation {
    // pub seed: *const u128,
    // planets: Vec<game_objects::Planet>,
    pub chunck_loader: ChunkLoader,
    pub main_camera: game_objects::Camera,
}

impl Simulation {
    pub fn start_simulation() -> Simulation{
        // const new_seed: u128 = 1;
        let camera = game_objects::Camera::new();
        // let planet = game_objects::Planet{
        //     position: game_objects::Position::new(),
        //     planetary_position: game_objects::CubicCoordinate::new(),
        //     seed: &new_seed,
        // };
        Simulation{chunck_loader: ChunkLoader::start_chunk_loader(&camera), main_camera: camera}
    }


}

pub struct ChunkLoader {
    loadedChunks: Vec<game_objects::Chunk>,
}

impl ChunkLoader {
    pub fn start_chunk_loader(camera: &game_objects::Camera) -> ChunkLoader{
        ChunkLoader{loadedChunks: vec![game_objects::Chunk::load_chunk(camera)]}
    }

    pub fn draw_chunks(&self, camera: &game_objects::Camera){
        // for i in self.loadedChunks {
            self.loadedChunks[0].draw_tiles(camera);
        // }
    }
}
