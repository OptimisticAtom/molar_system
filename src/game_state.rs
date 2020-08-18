extern crate nalgebra;
use std::time::{Duration, Instant};

mod game_objects;


pub struct Simulation {
    // pub seed: *const u128,
    // planets: Vec<game_objects::Planet>,
    pub chunck_loader: ChunkLoader,
    pub main_camera: game_objects::Camera,
    frame_time: Duration,
    last_frame: Instant,
    frames_per_step: u8,
    frames_elapsed_since_last_step: u8,
}

impl Simulation {
    pub fn start_simulation(fps: u8) -> Simulation{
        // const new_seed: u128 = 1;
        let fraction_of_second: f64 = 1.0 / fps as f64;
        let frame_as_microseconds: u64 = (fraction_of_second * 1e+6) as u64;
        let camera = game_objects::Camera::new();
        // let planet = game_objects::Planet{
        //     position: game_objects::Position::new(),
        //     planetary_position: game_objects::CubicCoordinate::new(),
        //     seed: &new_seed,
        // };
        Simulation{
            chunck_loader: ChunkLoader::start_chunk_loader(&camera),
            main_camera: camera,
            frame_time: Duration::from_micros(frame_as_microseconds),
            last_frame: Instant::now(),
            frames_per_step: fps / 10 as u8,
            frames_elapsed_since_last_step: 0,
        }
    }

    pub fn loop_call(&mut self){
        if self.last_frame.elapsed() >= self.frame_time{
            self.frame();
        }
    }

    fn frame(&mut self){
        if self.frames_elapsed_since_last_step >= self.frames_per_step{
            self.step();
            self.frames_elapsed_since_last_step = 0;
        }
        else{
            self.frames_elapsed_since_last_step += 1;
        }
    }

    fn step(&self){
        unsafe{game_objects::graphics::gl::Clear(game_objects::graphics::gl::COLOR_BUFFER_BIT);}
        self.chunck_loader.draw_chunks(&self.main_camera);
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
