extern crate nalgebra;
extern crate sdl2;
use sdl2::keyboard::Scancode;
use std::time::{Duration, Instant};
use std::collections::HashMap;


#[path ="graphics.rs"]
mod graphics;
mod game_objects;


pub struct Simulation {
    // pub seed: *const u128,
    planets: Vec<game_objects::Planet>,
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
            planets: vec![game_objects::Planet::new()],
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
        self.chunck_loader.draw_tiles(&self.main_camera);
    }

    pub fn handle_keyboard_event(&mut self, e: &mut sdl2::EventPump){
        if e.keyboard_state().is_scancode_pressed(Scancode::W) {self.main_camera.position.y += 1.0;}
        if e.keyboard_state().is_scancode_pressed(Scancode::A) {self.main_camera.position.x -= 1.0;}
        if e.keyboard_state().is_scancode_pressed(Scancode::S) {self.main_camera.position.y -= 1.0;}
        if e.keyboard_state().is_scancode_pressed(Scancode::D) {self.main_camera.position.x += 1.0;}
    }
}

pub struct ChunkLoader {
    loaded_chunks: HashMap<[i128; 3], game_objects::Chunk>,
    loaded_tiles: HashMap<[i128; 3], game_objects::EnviromentalTile>,
    player: Player,
}

impl ChunkLoader {
    pub fn new() -> ChunkLoader{
        ChunkLoader{loaded_chunks: HashMap::new(), loaded_tiles: HashMap::new(), player: Player::new()}
    }

    pub fn load_chunks(&mut self, camera: &game_objects::Camera,
        dictionary: &game_objects::chemistry::MaterialDictionary)
    {
        let render_distance: i128 = 2;
        let vicinity_x: i128 = (self.player.cubic_position.x / 100) + render_distance;
        let vicinity_y: i128 = (self.player.cubic_position.y / 100) + render_distance;
        let vicinity_z: i128 = (self.player.cubic_position.z / 100) + render_distance;
        for x in -vicinity_x..vicinity_x {
            for y in -vicinity_y..vicinity_y {
                for z in -vicinity_z..vicinity_z {
                    if x+y+z == 0 {
                        // let mut is_loaded = false;
                        // if self.loaded_chunks.len() > 0{
                        //     for chunk in &self.loaded_chunks {
                        //         if chunk.chunk_position.x == chunk_position.x &&
                        //         chunk.chunk_position.y == chunk_position.y &&
                        //         chunk.chunk_position.z == chunk_position.z{
                        //             is_loaded = true;
                        //         }
                        //     }
                        // }
                        let chunk_position = game_objects::CubicCoordinate{x: (x*100), y: (y*100), z: (z*100)};
                        let key = [chunk_position.x, chunk_position.y, chunk_position.z];
                        if !self.loaded_chunks.contains_key(&key){
                            self.loaded_chunks.insert(key, game_objects::Chunk{cubic_position: chunk_position});
                            self.load_chunk_of_tiles(camera, dictionary, chunk_position);
                            // self.load_chunk(chunk_position, dictionary, camera);
                            println!("loaded chunk: x{:?}, y{:?}, z{:?}", x, y, z);
                        }
                    }
                }
            }
        }
    }

    // pub fn load_chunk(&mut self, cubic_coordinate: game_objects::CubicCoordinate,
    //     dictionary: &game_objects::chemistry::MaterialDictionary, camera: &game_objects::Camera)
    // {
    //     self.loaded_chunks.insert(cubic_coordinate,
    //         game_objects::Chunk::load_chunk(camera, dictionary, cubic_coordinate));
    // }

    pub fn load_chunk_of_tiles(&mut self, camera: &game_objects::Camera, dictionary: &game_objects::chemistry::MaterialDictionary,
        chunk_coordinate: game_objects::CubicCoordinate){
        let chunk_size: i128 = game_objects::CHUNK_SIZE;
        let count = 0;
        for x in -chunk_size..chunk_size {
            for y in -chunk_size..chunk_size {
                for z in -chunk_size..chunk_size {
                    if x+y+z == 0 {
                        // if x.abs() == CHUNK_SIZE || y.abs() == CHUNK_SIZE || z.abs() == CHUNK_SIZE{
                        //
                        // }
                        let mut tile = game_objects::EnviromentalTile::spawn(
                            game_objects::CubicCoordinate{
                                x: (x + chunk_coordinate.x),
                                y: (y + chunk_coordinate.y),
                                z: (z + chunk_coordinate.z)
                            },
                            dictionary, camera);
                        // let r = (x as f32/50.0).abs();
                        // let g = (y as f32/50.0).abs();
                        // let b = (z as f32/50.0).abs();
                        tile.tile.hexagon.set_color(
                            tile.tile.molecule.color[0],
                            tile.tile.molecule.color[1],
                            tile.tile.molecule.color[2],
                            tile.tile.molecule.color[3]
                        );
                        // tiles.push(tile);
                        let key = [x + chunk_coordinate.x, y + chunk_coordinate.y, z + chunk_coordinate.z];
                        self.loaded_tiles.insert(key, tile);
                        println!("loaded tile{:?}", key);
                    }
                }
            }
        }
    }

    pub fn draw_tiles(&self, camera: &game_objects::Camera){
        for hash in &self.loaded_tiles {
            hash.1.tile.hexagon.render_hexagon(camera);
        }
    }
}

struct Player{
    pub cubic_position: game_objects::CubicCoordinate,
}

impl Player{
    pub fn new() -> Player{Player{cubic_position: game_objects::CubicCoordinate::new()}}
}
