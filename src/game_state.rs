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
use science::physics;


pub struct Simulation {
    // pub seed: *const u128,
    planets: Vec<basic::Planet>,
    pub chunck_loader: ChunkLoader,
    pub main_camera: basic::Camera,
    last_camera_state: basic::Camera,
    frame_time: Duration,
    last_frame: Instant,
    start_time: Instant,
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
            start_time: Instant::now(),
            frames_per_step: fps / 10 as u8,
            frames_elapsed_since_last_step: 0,
            global_step_count: 0,
            material_dictionary: chemistry::MaterialDictionary::new(),
        }
    }

    pub fn start_simulation(&mut self){
        self.chunck_loader.enviroment_renderer.initialize_object_renderer(include_str!("gen.vert"), include_str!("gen.geom"), include_str!("gen.frag"));
    }

    pub fn loop_call(&mut self){
        if self.last_frame.elapsed() >= self.frame_time{
            self.frame();
            let difference = self.last_frame.elapsed() - self.frame_time;
            self.last_frame = Instant::now() - difference;
            // println!("{:?}", difference);
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
        self.draw_tiles();
    }

    pub fn draw_tiles(&mut self){
        // println!("Drawing tiles");
        self.chunck_loader.enviroment_renderer.set_rotation(self.main_camera.rotation);
        self.chunck_loader.enviroment_renderer.set_float("scale", self.main_camera.scale as f32);
        let now = Instant::now();
        let time_since_start = now.duration_since(self.start_time);
        self.chunck_loader.enviroment_renderer.set_float("time", time_since_start.as_secs_f32());
        self.chunck_loader.enviroment_renderer.set_camera_position(
            self.main_camera.position.x as f32, self.main_camera.position.y as f32);

        let mut vertices: Vec<graphics::Vertex> = vec![];
        let cubic_position = basic::CubicCoordinate::position_to_cubic(self.main_camera.position);
        let scale = self.main_camera.scale as i128;
        for x in cubic_position.x - scale..cubic_position.x + scale + 1 {
            for y in std::cmp::max(cubic_position.y - scale, -x-(cubic_position.z + scale))..
            std::cmp::min(cubic_position.y + scale, -x-(cubic_position.z - scale)) + 1 {
                let z = -x-y;
                let hash = self.chunck_loader.loaded_tiles.get(&[x, y, z]);
                match hash{
                    None => {continue;},
                    _ => {}
                }
                let etile = hash.unwrap();
                let hex_vert = etile.creater_render_vertice(&self.main_camera);
                vertices.push(hex_vert);
            }
        }
        self.chunck_loader.enviroment_renderer.draw_object(vertices);
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
        let x: i128 = cubic_position.x - (cubic_position.x % 50);
        let y: i128 = cubic_position.y - (cubic_position.y % 50);
        let z: i128 = cubic_position.z - (cubic_position.z % 50);
        // for x in cubic_position.x - 50.. cubic_position.x + 51 {
        //     for y in std::cmp::max(cubic_position.y - 50, -x - (cubic_position.z + 50))..
        //     std::cmp::min(cubic_position.y + 50, -x - (cubic_position.z - 50)) + 1 {
        //         let z = -x-y;
        //         if x % 50 == 0 && y % 50 == 0 && z % 50 == 0 &&
        //         (x % 100 == 0 || y % 100 == 0 || z % 100 == 0){
        //
        //         }
        //     }
        // }
        if (x % 100 == 0 || y % 100 == 0 || z % 100 == 0) && x+y+z == 0{
            for neighboring_chunk in 0..7 {
                let neighbor = basic::CubicCoordinate::
                get_neighbor_chunk(basic::CubicCoordinate{x,y,z},
                    neighboring_chunk);

                let chunk_position = basic::CubicCoordinate{
                    x: (neighbor.x), y: (neighbor.y), z: (neighbor.z)};

                let key = [chunk_position.x, chunk_position.y, chunk_position.z];
                if !self.loaded_chunks.contains_key(&key){
                    self.loaded_chunks.insert(key, basic::Chunk{cubic_position: chunk_position});
                    self.load_chunk_of_tiles(camera, dictionary, chunk_position);
                    // self.load_chunk(chunk_position, dictionary, camera);
                    println!("loaded chunk: x{:?}, y{:?}, z{:?}", chunk_position.x, chunk_position.y, chunk_position.z);
                    println!("--------------------------------------------------");
                }
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
                // let mut tile = basic::EnviromentalTile::spawn(
                //     basic::CubicCoordinate{
                //         x: (x + chunk_coordinate.x),
                //         y: (y + chunk_coordinate.y),
                //         z: (z + chunk_coordinate.z)
                //     },
                //     dictionary, camera);
                // // let r = (x as f32/50.0).abs();
                // // let g = (y as f32/50.0).abs();
                // // let b = (z as f32/50.0).abs();
                // tile.tile.hexagon.set_color(
                //     tile.tile.molecule.color[0],
                //     tile.tile.molecule.color[1],
                //     tile.tile.molecule.color[2],
                //     tile.tile.molecule.color[3]
                // );
                let tile = ChunkLoader::generate_enviromental_tile(basic::CubicCoordinate{
                        x: (x + chunk_coordinate.x),
                        y: (y + chunk_coordinate.y),
                        z: (z + chunk_coordinate.z)
                    }, dictionary);
                // tiles.push(tile);
                let key = [x + chunk_coordinate.x, y + chunk_coordinate.y, z + chunk_coordinate.z];
                self.loaded_tiles.insert(key, tile);
                // println!("loaded tile{:?}", key);
            }
        }
    }

    pub fn generate_enviromental_tile(cubic_coordinate: basic::CubicCoordinate,
    dictionary: &chemistry::MaterialDictionary) -> basic::EnviromentalTile{
        let cubic_distance_from_center =
        basic::CubicCoordinate::distance(&cubic_coordinate, &basic::CubicCoordinate::new());

        let position = basic::Position::cubic_to_position(&cubic_coordinate);

        let cartesian_distance_from_center =
        basic::Position::distance(&position, &basic::Position::new());
        let mut formula = "".to_string();
        let mut kelvin = 0.0;
        let mut mols = 1.0;
        let mut color: [f32; 4] = [0.1,0.1,0.7,1.0];
        let mut state_of_matter: u8 = 0;
        let mut pp = physics::PhysicalProperties::new();
        if cartesian_distance_from_center < 500.0{
            formula = "silica".to_string();
            kelvin = 2500.0;
            mols = 192.181;
            color = [0.9,0.6,0.1,1.0];
            state_of_matter = 254;
        }
        else if cartesian_distance_from_center < 800.0{
            formula = "silica".to_string();
            kelvin = 300.0;
            mols = 80.256;
            color = [0.6,0.5,0.5,1.0];
            state_of_matter = 0;
        }
        else if cartesian_distance_from_center < 900.0{
            formula = "silica".to_string();
            kelvin = 250.0;
            mols = 24.60;
            color = [0.7,0.6,0.4,1.0];
            pp.max_grain_size = 0.1;
        }
        else if cartesian_distance_from_center < 950.0{
            formula = "air".to_string();
            kelvin = 250.0;
            mols = 42.87;
            color = [0.7,0.8,0.9,0.8];
            state_of_matter = 255;
        }
        else{
            formula = "air".to_string();
            kelvin = 100.0;
            mols = 5.07;
            color = [0.0,0.0,0.0,0.0];
            state_of_matter = 255;
        }
        let mut tile = basic::EnviromentalTile{
         tile: basic::Tile::new(formula, &position),
         cubic_position: cubic_coordinate,
         material_state: chemistry::MaterialState{kelvin, mols, state_of_matter},
         physical_properties: pp,
        };
        tile.tile.hexagon.color = color;
        tile
    }
}


struct Player{
    pub cubic_position: basic::CubicCoordinate,
}

impl Player{
    pub fn new() -> Player{Player{cubic_position: basic::CubicCoordinate::new()}}
}
