extern crate nalgebra;
use std::time::{Duration, Instant};
#[path ="graphics.rs"]
mod graphics;
mod game_objects;


pub struct Simulation {
    // pub seed: *const u128,
    // planets: Vec<game_objects::Planet>,
    pub chunck_loader: ChunkLoader,
    pub main_camera: game_objects::Camera,
    last_camera_position: game_objects::Position,
    last_camera_scale: f64,
    frame_time: Duration,
    last_frame: Instant,
    frames_per_step: u8,
    frames_elapsed_since_last_step: u8,
    global_step_count: u128,
    material_dictionary: game_objects::chemistry::MaterialDictionary,
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
            chunck_loader: ChunkLoader::new(),
            main_camera: camera,
            last_camera_position: game_objects::Position::new(),
            last_camera_scale: 50.0,
            frame_time: Duration::from_micros(frame_as_microseconds),
            last_frame: Instant::now(),
            frames_per_step: fps / 10 as u8,
            frames_elapsed_since_last_step: 0,
            global_step_count: 0,
            material_dictionary: game_objects::chemistry::MaterialDictionary::new(),
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
            // println!("{:?}", self.frames_elapsed_since_last_step);
        }
        if self.main_camera.position.x != self.last_camera_position.x ||
        self.main_camera.position.y != self.last_camera_position.y ||
        self.frames_elapsed_since_last_step == 0 ||
        self.main_camera.scale != self.last_camera_scale{
            self.draw_new_screen();
            self.last_camera_position = self.main_camera.position;
            self.last_camera_scale = self.main_camera.scale;
        }
    }

    fn step(&mut self){
        self.global_step_count += 1;
        // println!("step: {:?}", self.global_step_count);
        self.chunck_loader.load_chunks(&self.main_camera, &self.material_dictionary)
    }

    fn draw_new_screen(&self){
        unsafe{graphics::gl::Clear(graphics::gl::COLOR_BUFFER_BIT);}
        self.chunck_loader.draw_chunks(&self.main_camera);
    }
}

pub struct ChunkLoader {
    loaded_chunks: Vec<game_objects::Chunk>,
}

impl ChunkLoader {
    pub fn new() -> ChunkLoader{
        ChunkLoader{loaded_chunks: vec![]}
    }

    pub fn load_chunks(&mut self, camera: &game_objects::Camera,
        dictionary: &game_objects::chemistry::MaterialDictionary)
    {

        if self.loaded_chunks.len() < 1 {
            self.loaded_chunks.push(game_objects::Chunk::load_chunk(camera, dictionary));
        }

    }

    pub fn draw_chunks(&self, camera: &game_objects::Camera){
        // for i in self.loadedChunks {
            self.loaded_chunks[0].draw_tiles(camera);
        // }
    }
}
