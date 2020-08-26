// extern crate nalgebra;
// extern crate sdl2;
// use sdl2::keyboard::Scancode;
// use std::time::{Duration, Instant};
// use std::collections::HashMap;
//
// extern crate gl_api;
// use gl_api::graphics;
// extern crate game_objects;
// use game_objects::basic;
//
//
// pub struct Canvas {
//     // pub seed: *const u128,
//     planets: Vec<basic::Planet>,
//     pub main_camera: basic::Camera,
//     last_camera_state: basic::Camera,
//     hexagons: HashMap<[i16; 3], basic::Hexagon>,
//     frame_time: Duration,
//     last_frame: Instant,
//     frames_per_step: u8,
//     frames_elapsed_since_last_step: u8,
//     global_step_count: u128,
//     material_dictionary: chemistry::MaterialDictionary,
// }
//
// impl Canvas {
//     pub fn new(fps: u8) -> Canvas{
//         // const new_seed: u128 = 1;
//         let fraction_of_second: f64 = 1.0 / fps as f64;
//         let frame_as_microseconds: u64 = (fraction_of_second * 1e+6) as u64;
//         let camera = basic::Camera::new();
//         // let planet = basic::Planet{
//         //     position: basic::Position::new(),
//         //     planetary_position: basic::CubicCoordinate::new(),
//         //     seed: &new_seed,
//         // };
//         Canvas{
//             planets: vec![basic::Planet::new()],
//             main_camera: camera,
//             last_camera_state: basic::Camera::new(),
//             hexagons: HashMap::new(),
//             frame_time: Duration::from_micros(frame_as_microseconds),
//             last_frame: Instant::now(),
//             frames_per_step: fps / 10 as u8,
//             frames_elapsed_since_last_step: 0,
//             global_step_count: 0,
//             material_dictionary: chemistry::MaterialDictionary::new(),
//         }
//     }
//
//     pub fn start_simulation(&mut self){
//         self.chunck_loader.enviroment_renderer.initialize_object_renderer();
//     }
//
//     pub fn loop_call(&mut self){
//         if self.last_frame.elapsed() >= self.frame_time{
//             self.frame();
//         }
//     }
//
//     fn frame(&mut self){
//         if self.main_camera.position.x != self.last_camera_state.position.x ||
//         self.main_camera.position.y != self.last_camera_state.position.y ||
//         self.frames_elapsed_since_last_step == 0 ||
//         self.main_camera.scale != self.last_camera_state.scale ||
//         self.main_camera.rotation != self.last_camera_state.rotation{
//             self.draw_new_screen();
//             self.last_camera_state.position = self.main_camera.position;
//             self.last_camera_state.scale = self.main_camera.scale;
//             self.last_camera_state.rotation = self.main_camera.rotation;
//         }
//     }
//
//     fn draw_new_screen(&mut self){
//         unsafe{graphics::gl::Clear(graphics::gl::COLOR_BUFFER_BIT);}
//         self.chunck_loader.draw_tiles(&self.main_camera);
//     }
//
//     pub fn handle_keyboard_event(&mut self, e: &mut sdl2::EventPump){
//         if e.keyboard_state().is_scancode_pressed(Scancode::W) {self.main_camera.position.y += 1.0;}
//         if e.keyboard_state().is_scancode_pressed(Scancode::A) {self.main_camera.position.x -= 1.0;}
//         if e.keyboard_state().is_scancode_pressed(Scancode::S) {self.main_camera.position.y -= 1.0;}
//         if e.keyboard_state().is_scancode_pressed(Scancode::D) {self.main_camera.position.x += 1.0;}
//         if e.keyboard_state().is_scancode_pressed(Scancode::Q) {self.main_camera.rotation += 0.1;}
//         if e.keyboard_state().is_scancode_pressed(Scancode::E) {self.main_camera.rotation -= 0.1;}
//     }
// }
//
//
// pub fn draw_tiles(&mut self, camera: &basic::Camera){
//     // let range = camera.scale as i128;
//     // let range_x = (self.camera.position.x + range);
//     // let range_z = (self.player.cubic_position.z + range);
//     // for x in -range_x..range_x {
//     //     for z in std::cmp::max(-range_z, -x-range_z)..std::cmp::min(range_z, -x+range_z) {
//     //         let y = -x-z;
//     //         let option = self.loaded_tiles.get(&[x,y,z]);
//     //         if option.is_some(){
//     //             let tile = option.unwrap();
//     //             tile.tile.hexagon.render_hexagon(camera);
//     //         }
//     //         // tile.tile.hexagon.render_hexagon(camera);
//     //     }
//     // }
//     self.enviroment_renderer.set_transform(camera.rotation);
//     let mut vertices: Vec<graphics::Vertex> = vec![];
//     let mut indices: Vec<u32> = vec![];
//     for hash in &self.loaded_tiles {
//         let hex_verts = hash.1.tile.hexagon.creater_render_vertices(camera);
//         let mut hex_indices = [0u32; 12];
//         for i in 0..6 {
//             let vert = vertices.len() as u32;
//             vertices.push(hex_verts[i]);
//             match i{
//                 0 => {hex_indices[0] = vert; hex_indices[3] = vert;},
//                 1 => {hex_indices[4] = vert; hex_indices[6] = vert; hex_indices[9] = vert;},
//                 2 => {hex_indices[10] = vert},
//                 3 => {hex_indices[7] = vert; hex_indices[11] = vert;},
//                 4 => {hex_indices[1] = vert; hex_indices[5] = vert; hex_indices[8] = vert;},
//                 5 => {hex_indices[2] = vert},
//                 _ => {println!("game_state::draw_tiles - vertices are out of bounds{:?}", i);}
//             }
//         }
//         for i in 0..12 {
//             indices.push(hex_indices[i]);
//         }
//     }
//     self.enviroment_renderer.draw_object(vertices, indices);
// }
