extern crate gl_api;
use gl_api::graphics;
extern crate science;
use science::chemistry;
use std::f32::consts::PI;

pub const CHUNK_SIZE: i128 = 50;


#[derive(Copy, Clone)]
pub struct Position{
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new() -> Position{
        Position{x: 0.0, y: 0.0}
    }

    pub fn axial_to_position(axial_coordinate: &AxialCoordinate) -> Position{
        let mat: nalgebra::Matrix2<f64> =
        nalgebra::Matrix2::new(1.732050808, 0.8660254038,0.0,1.5);
        //--------------
        let mat2: nalgebra::Matrix2x1<f64> =
        nalgebra::Matrix2x1::new(axial_coordinate.q as f64, axial_coordinate.r as f64);
        //--------------
        let mat3: nalgebra::Matrix2x1<f64> = 1.0/1.732050808 * mat * mat2;

        let mat4: nalgebra::Matrix2x1<f64> = mat3;

        Position{x: mat4.data[0], y: -mat4.data[1]}
    }

    pub fn cubic_to_position(cubic_coordinate: &CubicCoordinate) -> Position{
        Position::axial_to_position(&AxialCoordinate::cubic_to_axial(cubic_coordinate))
    }

    pub fn distance(pos1: &Position, pos2: &Position) -> f64{
        ((pos1.x - pos2.x).abs().powi(2) + (pos1.y - pos2.y).abs().powi(2)).sqrt()
    }
}

pub struct NormalizedPosition {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone)]
pub struct CubicCoordinate{
    pub x: i128,
    pub y: i128,
    pub z: i128,
}

impl CubicCoordinate {
    pub fn new() -> CubicCoordinate{
        CubicCoordinate{x: 0, y: 0, z: 0}
    }

    pub fn axial_to_cubic(axial_coordinate: &AxialCoordinate) -> CubicCoordinate{
        let x = axial_coordinate.q;
        let z = axial_coordinate.r;
        let y = -x-z;
        CubicCoordinate{x,y,z}
    }

    pub fn position_to_cubic(position: Position) -> CubicCoordinate{
        CubicCoordinate::axial_to_cubic(&AxialCoordinate::position_to_axial(position))
    }

    pub fn distance(coord1: &CubicCoordinate, coord2: &CubicCoordinate) -> u128{
        ((coord1.x - coord2.x).abs() +
        (coord1.y - coord2.y).abs() + (coord1.z - coord2.z).abs()) as u128
    }

    pub fn get_neighbor(cubic_coordinate: CubicCoordinate, neighbor: i8) -> CubicCoordinate{
        match neighbor {
            0 => {CubicCoordinate{
                x: (cubic_coordinate.x + 1), y: cubic_coordinate.y, z: (cubic_coordinate.z - 1)}},
            1 => {CubicCoordinate{
                x: (cubic_coordinate.x + 1), y: (cubic_coordinate.y - 1), z: cubic_coordinate.z}},
            2 => {CubicCoordinate{
                x: cubic_coordinate.x, y: (cubic_coordinate.y - 1), z: (cubic_coordinate.z + 1)}},
            3 => {CubicCoordinate{
                x: (cubic_coordinate.x - 1), y: cubic_coordinate.y, z: (cubic_coordinate.z + 1)}},
            4 => {CubicCoordinate{
                x: (cubic_coordinate.x - 1), y: (cubic_coordinate.y + 1), z: cubic_coordinate.z}},
            5 => {CubicCoordinate{
                x: cubic_coordinate.x, y: (cubic_coordinate.y + 1), z: (cubic_coordinate.z - 1)}},
            6 => {cubic_coordinate},
        _ => {println!("hexagon at {:?},{:?},{:?} asked for neighbor {:?} which is not an option"
            , cubic_coordinate.x, cubic_coordinate.y, cubic_coordinate.z, neighbor);
            CubicCoordinate::new()}
        }
    }

    pub fn get_neighbor_chunk(cubic_coordinate: CubicCoordinate, neighbor: i8) -> CubicCoordinate{
        match neighbor {
            0 => {CubicCoordinate{
                x: (cubic_coordinate.x + 50), y: (cubic_coordinate.y + 50), z: (cubic_coordinate.z - 100)}},
            1 => {CubicCoordinate{
                x: (cubic_coordinate.x + 100), y: (cubic_coordinate.y- 50), z: (cubic_coordinate.z - 50)}},
            2 => {CubicCoordinate{
                x: cubic_coordinate.x + 50, y: (cubic_coordinate.y - 100), z: cubic_coordinate.z + 50}},
            3 => {CubicCoordinate{
                x: (cubic_coordinate.x - 50), y: (cubic_coordinate.y - 50), z: (cubic_coordinate.z + 100)}},
            4 => {CubicCoordinate{
                x: (cubic_coordinate.x - 100), y: (cubic_coordinate.y + 50), z: (cubic_coordinate.z + 50)}},
            5 => {CubicCoordinate{
                x: (cubic_coordinate.x - 50), y: cubic_coordinate.y + 100, z: (cubic_coordinate.z - 50)}},
            6 => {cubic_coordinate},
        _ => {println!("hexagon at {:?},{:?},{:?} asked for neighbor {:?} which is not an option"
            , cubic_coordinate.x, cubic_coordinate.y, cubic_coordinate.z, neighbor);
            CubicCoordinate::new()}
        }
    }
}

pub struct AxialCoordinate {
    q: i128,
    r: i128,
}

impl AxialCoordinate {
    pub fn new() -> AxialCoordinate{
        AxialCoordinate{q: 0, r: 0}
    }

    pub fn cubic_to_axial(cubic_coordinate: &CubicCoordinate) -> AxialCoordinate{
        let new_q = cubic_coordinate.x;
        let new_r = cubic_coordinate.z;
        AxialCoordinate{q: new_q, r: new_r}
    }

    pub fn position_to_axial(position: Position) -> AxialCoordinate{
        let q = ((1.732050808/3.0 * position.x - 1.0/3.0 * -position.y) / (1.0/1.732050808)) as i128;
        let r = ((2.0 / 3.0 * -position.y) / (1.0/1.732050808)) as i128;

        AxialCoordinate{q, r}
    }
}


pub struct Hexagon{
    pub position: Position,
    pub color: [f32; 4],
    // pub renderer: graphics::Renderer,
}

impl Hexagon{
    pub fn new() -> Hexagon{
        // let pos = Position::set_position(x, y);
        // let normalized = Hexagon::world_space_to_screen_space(&pos, &camera);
        // let vertices = Hexagon::normalized_vertex_array(&normalized, &camera);
        // let rend = graphics::Renderer::initialize_object_renderer(vertices);
        // Hexagon{position: pos, renderer: rend}
        Hexagon{
            position: Position::new(),
            color: [0.2, 0.8, 0.2, 1.0],
            // renderer: graphics::Renderer::new(),
        }
    }

    pub fn initialize_hexagon(set_position: &Position, camera: &Camera) -> Hexagon{
        let mut hexagon = Hexagon::new();
        hexagon.position = *set_position;
        // hexagon.renderer.initialize_object_renderer(hexagon.creater_render_vertices(camera));
        hexagon
    }

    pub fn world_space_to_screen_space(position: &Position, camera: &Camera) -> NormalizedPosition{
        let distance_x: f64 = position.x - camera.position.x;
        let distance_y: f64 = position.y  - camera.position.y;
        let normalized_x: f32 = (distance_x / camera.scale) as f32;
        let normalized_y: f32 = (distance_y / camera.scale) as f32;
        NormalizedPosition{x: normalized_x, y: normalized_y}
    }

    pub fn normalized_vertex_positions(position: &NormalizedPosition, camera: &Camera)->[[f32;2];6]
    {
        let scale = camera.scale as f32;
        let distance_x = 0.5 / scale;
        let distance_y = 0.2886751346 / scale;
        [
            [position.x, (position.y + (0.5773502692 / scale))],
            [(position.x + distance_x), (position.y + distance_y)],
            [(position.x + distance_x), (position.y - distance_y)],
            [position.x, (position.y - (0.5773502692 / scale))],
            [(position.x - distance_x), (position.y - distance_y)],
            [(position.x - distance_x), (position.y + distance_y)],
        ]
    }

    pub fn transform_vertex_positions(vertex_positions:[[f32; 2]; 6],camera:&Camera)->[[f32;2]; 6]{
        let theta = camera.rotation;
        let rotation_matrix: nalgebra::Matrix2<f32> =
        nalgebra::Matrix2::new(theta.cos(), -(theta.sin()), theta.sin(), theta.cos());
        let mut transformed_positions: [[f32; 2]; 6] = [[0.0; 2]; 6];
        for i in 0..6 {
            let position_matrix = nalgebra::Matrix2x1::new(vertex_positions[i][0], vertex_positions[i][1]);
            let transformed_matrix = rotation_matrix * position_matrix;
            transformed_positions[i] = [transformed_matrix.data[0], transformed_matrix.data[1]];
        }
        transformed_positions
    }

    pub fn creater_render_vertices(&self, camera: &Camera) -> [graphics::Vertex; 6]{
        let normalized_position: NormalizedPosition = Hexagon::world_space_to_screen_space(
            &self.position, camera);
        // if normalized_position.x > 1.1 || normalized_position.x < -1.1 ||
        // normalized_position.y > 1.1 || normalized_position.y < -1.1{
        //     return vec![];
        // }
        let vertex_positions = Hexagon::normalized_vertex_positions(&normalized_position, camera);
        // let new_vertex_positions = Hexagon::transform_vertex_positions(vertex_positions, camera);

        [
        graphics::Vertex{position: vertex_positions[0], color: self.color},
        graphics::Vertex{position: vertex_positions[1], color: self.color},
        graphics::Vertex{position: vertex_positions[2], color: self.color},
        graphics::Vertex{position: vertex_positions[3], color: self.color},
        graphics::Vertex{position: vertex_positions[4], color: self.color},
        graphics::Vertex{position: vertex_positions[5], color: self.color},
        ]
    }

    // pub fn render_hexagon(&self, camera: &Camera){
    //     self.renderer.draw_object(self.creater_render_vertices(camera));
    // }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32){
        self.color = [r, g, b, a];
    }
}

pub struct Camera {
    pub position: Position,
    pub scale: f64,
    pub rotation: f32,
}

impl Camera {
    pub fn new() -> Camera{
        Camera{
            position: Position::new(),
            scale: 50.0,
            rotation: 0.0,
        }
    }

    pub fn zoom(&mut self, y: i32){
        let zoom_amount = self.scale / 100.0;
        self.scale += zoom_amount * y as f64;
    }
}

pub struct Tile {
    pub hexagon: Hexagon,
    formula: String,
    pub molecule: chemistry::Molecule,
}

impl Tile {
    pub fn new(assigned_formula: String, camera: &Camera,
        position: &Position, dictionary: &chemistry::MaterialDictionary) -> Tile{
        let new_hexagon = Hexagon::initialize_hexagon(position, camera);
        //---------------
        let new_molecule = dictionary.access_dictionary(&assigned_formula);
        Tile{
            hexagon: new_hexagon,
            formula: assigned_formula,
            molecule: new_molecule,
        }
    }
}

pub struct EnviromentalTile {
    pub tile: Tile,
    cubic_position: CubicCoordinate,
}

impl EnviromentalTile {
    pub fn spawn(cubic_coordinate: CubicCoordinate, dictionary: &chemistry::MaterialDictionary,
        camera: &Camera) -> EnviromentalTile
    {
        let position = Position::cubic_to_position(&cubic_coordinate);
        let formula = EnviromentalTile::decide_formula(&cubic_coordinate, &position);


        let new_tile = Tile::new(formula, camera, &position, dictionary);
        EnviromentalTile{
            tile: new_tile,
            cubic_position: cubic_coordinate,
        }
    }

    pub fn decide_formula(cubic_coordinate: &CubicCoordinate, position: &Position) -> String{
        let cubic_distance_from_center =
        CubicCoordinate::distance(cubic_coordinate, &CubicCoordinate::new());

        let cartesian_distance_from_center = Position::distance(position, &Position::new());

        if cartesian_distance_from_center < 500.0{
            return  "magma".to_string();
        }
        else if cartesian_distance_from_center < 800.0{
            return "stone".to_string();
        }
        else if cartesian_distance_from_center < 900.0{
            return "dirt".to_string();
        }
        else if cartesian_distance_from_center < 950.0{
            return "air".to_string();
        }
        else{
            return "vacum".to_string();
        }
    }
}

pub struct TileState{

}

pub struct Chunk {
    // enviromental_tiles: Vec<EnviromentalTile>,
    pub cubic_position: CubicCoordinate,
}

// impl Chunk {
//     pub fn load_chunk(camera: &Camera, dictionary: &chemistry::MaterialDictionary,
//         chunk_coordinate: CubicCoordinate) -> Chunk
//     {
//         // let mut tiles: Vec<EnviromentalTile> = vec![];
//
//         Chunk{cubic_position: CubicCoordinate::new()}
//     }
//
//     pub fn draw_tiles(&self, camera: &Camera){
//         for tile in &self.enviromental_tiles {
//             if ((tile.tile.hexagon.position.x - camera.position.x) / camera.scale).abs() < 1.1 ||
//             ((tile.tile.hexagon.position.y - camera.position.y) / camera.scale).abs() < 1.1{
//                 tile.tile.hexagon.render_hexagon(camera);
//             }
//
//         }
//     }
// }

pub struct Planet {
    // position: Position,
    // snap_position: OffsetCoordinate,
    planetary_position: CubicCoordinate,
    // seed: *const u128,
}

impl Planet{
    pub fn new() -> Planet{
        Planet{planetary_position: CubicCoordinate::new()}
    }
}
