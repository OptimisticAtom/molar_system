extern crate nalgebra;
extern crate sdl2;
use sdl2::keyboard::Scancode;
use std::time::{Duration, Instant};
use std::collections::HashMap;

extern crate gl_api;
use gl_api::graphics;
extern crate game_objects;
use game_objects::basic;
extern crate science;
use science::chemistry;


pub struct Simulation {
    // pub seed: *const u128,
    planets: Vec<basic::Planet>,
    pub chunck_loader: ChunkLoader,
    pub main_camera: basic::Camera,
    last_camera_state: basic::Camera,
    frame_time: Duration,
    last_frame: Instant,
    time_since_start: Instant,
    frames_per_step: u8,
    frames_elapsed_since_last_step: u8,
    global_step_count: u128,
    material_dictionary: chemistry::MaterialDictionary,
}

impl Simulation {
    pub fn new(fps: u8) -> Simulation{
        // const new_seed: u128 = 1;
        let fraction_of_second: f64 = 1.0 / fps as f64;
        let frame_as_microseconds: u64 = (fraction_of_second * 1e+6) as u64;
        let camera = basic::Camera::new();
        // let planet = basic::Planet{
        //     position: basic::Position::new(),
        //     planetary_position: basic::CubicCoordinate::new(),
        //     seed: &new_seed,
        // };
        Simulation{
            planets: vec![basic::Planet::new()],
            chunck_loader: ChunkLoader::new(),
            main_camera: camera,
            last_camera_state: basic::Camera::new(),
            frame_time: Duration::from_micros(frame_as_microseconds),
            last_frame: Instant::now(),
            time_since_start: Instant::now(),
            frames_per_step: fps / 10 as u8,
            frames_elapsed_since_last_step: 0,
            global_step_count: 0,
            material_dictionary: chemistry::MaterialDictionary::new(),
        }
    }

    pub fn start_simulation(&mut self){
        self.chunck_loader.enviroment_renderer.initialize_object_renderer(include_str!("gen.vert"), include_str!("gen.frag"));
    }

    pub fn loop_call(&mut self){
        if self.last_frame.elapsed() >= self.frame_time{
            self.frame();
            let difference = self.last_frame.elapsed() - self.frame_time;
            self.last_frame = Instant::now() - difference;
            println!("{:?}", difference);
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
        if self.main_camera.position.x != self.last_camera_state.position.x ||
        self.main_camera.position.y != self.last_camera_state.position.y ||
        self.frames_elapsed_since_last_step == 0 ||
        self.main_camera.scale != self.last_camera_state.scale ||
        self.main_camera.rotation != self.last_camera_state.rotation{
            self.draw_new_screen();
            self.last_camera_state.position = self.main_camera.position;
            self.last_camera_state.scale = self.main_camera.scale;
            self.last_camera_state.rotation = self.main_camera.rotation;
        }
    }

    fn step(&mut self){
        self.global_step_count += 1;
        // println!("step: {:?}", self.global_step_count);
        self.chunck_loader.load_chunks(&self.main_camera, &self.material_dictionary)
    }

    fn draw_new_screen(&mut self){
        unsafe{graphics::gl::Clear(graphics::gl::COLOR_BUFFER_BIT);}
        self.chunck_loader.draw_tiles(&self.main_camera);
    }

    pub fn handle_keyboard_event(&mut self, e: &mut sdl2::EventPump){
        if e.keyboard_state().is_scancode_pressed(Scancode::W) {self.main_camera.position.y += 1.0;}
        if e.keyboard_state().is_scancode_pressed(Scancode::A) {self.main_camera.position.x -= 1.0;}
        if e.keyboard_state().is_scancode_pressed(Scancode::S) {self.main_camera.position.y -= 1.0;}
        if e.keyboard_state().is_scancode_pressed(Scancode::D) {self.main_camera.position.x += 1.0;}
        if e.keyboard_state().is_scancode_pressed(Scancode::Q) {self.main_camera.rotation += 0.1;}
        if e.keyboard_state().is_scancode_pressed(Scancode::E) {self.main_camera.rotation -= 0.1;}
    }
}

pub struct ChunkLoader {
    loaded_chunks: HashMap<[i128; 3], basic::Chunk>,
    loaded_tiles: HashMap<[i128; 3], basic::EnviromentalTile>,
    enviroment_renderer: graphics::Renderer,
    player: Player,
}

impl ChunkLoader {
    pub fn new() -> ChunkLoader{
        ChunkLoader{
            loaded_chunks: HashMap::new(),
            loaded_tiles: HashMap::new(),
            enviroment_renderer: graphics::Renderer::new(),
            player: Player::new()}
    }

    pub fn load_chunks(&mut self, camera: &basic::Camera,
        dictionary: &chemistry::MaterialDictionary)
    {
        // let render_distance: i128 = 2;
        let cubic_position = basic::CubicCoordinate::position_to_cubic(camera.position);
        let chunk_x: i128 = cubic_position.x - (cubic_position.x % 100);
        let chunk_y: i128 = cubic_position.y - (cubic_position.y % 100);
        let chunk_z: i128 = cubic_position.z - (cubic_position.z % 100);
        for neighboring_chunk in 0..7 {
            let neighbor = basic::CubicCoordinate::
            get_neighbor_chunk(basic::CubicCoordinate{x: chunk_x, y: chunk_y, z: chunk_z},
                neighboring_chunk);

            let chunk_position = basic::CubicCoordinate{
                x: (neighbor.x), y: (neighbor.y), z: (neighbor.z)};

            let key = [chunk_position.x, chunk_position.y, chunk_position.z];
            if !self.loaded_chunks.contains_key(&key){
                self.loaded_chunks.insert(key, basic::Chunk{cubic_position: chunk_position});
                self.load_chunk_of_tiles(camera, dictionary, chunk_position);
                // self.load_chunk(chunk_position, dictionary, camera);
                println!("loaded chunk: x{:?}, y{:?}, z{:?}", chunk_position.x, chunk_position.y, chunk_position.z);
            }
        }
    }

    pub fn load_chunk_of_tiles(&mut self, camera: &basic::Camera, dictionary: &chemistry::MaterialDictionary,
        chunk_coordinate: basic::CubicCoordinate){
        let chunk_size: i128 = basic::CHUNK_SIZE;
        let count = 0;
        for x in -chunk_size..chunk_size + 1 {
            for y in std::cmp::max(-chunk_size, -x-chunk_size)..std::cmp::min(chunk_size, -x+chunk_size) + 1 {
                let z = -x-y;
                // if x.abs() == CHUNK_SIZE || y.abs() == CHUNK_SIZE || z.abs() == CHUNK_SIZE{
                //
                // }
                let mut tile = basic::EnviromentalTile::spawn(
                    basic::CubicCoordinate{
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
                // println!("loaded tile{:?}", key);
            }
        }
    }

    pub fn draw_tiles(&mut self, camera: &basic::Camera){
        // let range = camera.scale as i128;
        // let range_x = (self.camera.position.x + range);
        // let range_z = (self.player.cubic_position.z + range);
        // for x in -range_x..range_x {
        //     for z in std::cmp::max(-range_z, -x-range_z)..std::cmp::min(range_z, -x+range_z) {
        //         let y = -x-z;
        //         let option = self.loaded_tiles.get(&[x,y,z]);
        //         if option.is_some(){
        //             let tile = option.unwrap();
        //             tile.tile.hexagon.render_hexagon(camera);
        //         }
        //         // tile.tile.hexagon.render_hexagon(camera);
        //     }
        // }
        self.enviroment_renderer.set_rotation(camera.rotation);
        self.enviroment_renderer.set_scale(camera.scale as f32);
        let mut vertices: Vec<graphics::Vertex> = vec![];
        let mut indices: Vec<u32> = vec![];
        // for hash in &self.loaded_tiles {
        //
        // }
        let cubic_position = basic::CubicCoordinate::position_to_cubic(camera.position);
        let scale = camera.scale as i128;
        for x in cubic_position.x - 50..cubic_position.x + 51 {
            for y in std::cmp::max(cubic_position.y - 50, -x-(cubic_position.z + 50))..
            std::cmp::min(cubic_position.y + 50, -x-(cubic_position.z - 50)) + 1 {
                let z = -x-y;
                let hash = self.loaded_tiles.get(&[x, y, z]);
                match hash{
                    None => {continue;},
                    _ => {}
                }
                let etile = hash.unwrap();
                let hex_verts = etile.tile.hexagon.creater_render_vertices(camera);
                let mut hex_indices = [0u32; 12];
                for i in 0..6 {
                    let vert = vertices.len() as u32;
                    vertices.push(hex_verts[i]);
                    match i{
                        0 => {hex_indices[0] = vert; hex_indices[3] = vert;},
                        1 => {hex_indices[4] = vert; hex_indices[6] = vert; hex_indices[9] = vert;},
                        2 => {hex_indices[10] = vert},
                        3 => {hex_indices[7] = vert; hex_indices[11] = vert;},
                        4 => {hex_indices[1] = vert; hex_indices[5] = vert; hex_indices[8] = vert;},
                        5 => {hex_indices[2] = vert},
                        _ => {println!("game_state::draw_tiles - vertices are out of bounds{:?}", i);}
                    }
                }
                for i in 0..12 {
                    indices.push(hex_indices[i]);
                }
            }
        }
        self.enviroment_renderer.draw_object(vertices, indices);
    }
}

struct Player{
    pub cubic_position: basic::CubicCoordinate,
}

impl Player{
    pub fn new() -> Player{Player{cubic_position: basic::CubicCoordinate::new()}}
}
