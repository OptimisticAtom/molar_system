#[path =".././graphics.rs"]
mod graphics;
mod chemistry;

const COEFFICIENT_OF_X: f64 = 0.5;
const COEFFICIENT_OF_Y: f64 = 0.8660254038;

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
        let mat: nalgebra::Matrix2<f64> = nalgebra::Matrix2::new(1.732050808, 0.8660254038,0.0,1.5);
        let mat2: nalgebra::Matrix2x1<f64> = nalgebra::Matrix2x1::new(axial_coordinate.r as f64, axial_coordinate.q as f64);
        let mat3: nalgebra::Matrix2x1<f64> = 1.0/1.732050808 * mat * mat2;
        let mat4: nalgebra::Matrix2x1<f64> = mat3;
        Position{x: mat4.data[0], y: mat4.data[1]}
    }
}

pub struct NormalizedPosition {
    x: f32,
    y: f32,
}

pub struct CubicCoordinate{
    x: i128,
    y: i128,
    z: i128,
}

impl CubicCoordinate {
    pub fn new() -> CubicCoordinate{
        CubicCoordinate{x: 0, y: 0, z: 0}
    }

    // pub fn axial_to_cubic(axial_coordinate: &AxialCoordinate) -> CubicCoordinate{
    //
    // }
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
}


pub struct Hexagon{
    pub position: Position,
    pub renderer: graphics::Renderer,
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
            renderer: graphics::Renderer::new(),
        }
    }

    pub fn initialize_hexagon(set_position: Position, camera: &Camera) -> Hexagon{
        let mut hexagon = Hexagon::new();
        hexagon.position = set_position;
        hexagon.renderer.initialize_object_renderer(hexagon.creater_render_vertices(camera));
        hexagon
    }

    pub fn world_space_to_screen_space(position: &Position, camera: &Camera) -> NormalizedPosition{
        let distance_x: f64 = position.x - camera.position.x;
        let distance_y: f64 = position.y  - camera.position.y;
        let normalized_x: f32 = (distance_x / camera.scale) as f32;
        let normalized_y: f32 = (distance_y / camera.scale) as f32;
        NormalizedPosition{x: normalized_x, y: normalized_y}
    }

    pub fn normalized_vertex_array(position: &NormalizedPosition, camera: &Camera) -> Vec<f32>{
        let scale = camera.scale as f32;
        let distance_x = 0.5 / scale;
        let distance_y = 0.2886751346 / scale;
        vec![
        position.x, (position.y + (0.5773502692 / scale)),
        (position.x + distance_x), (position.y + distance_y),
        (position.x + distance_x), (position.y - distance_y),
        position.x, (position.y - (0.5773502692 / scale)),
        (position.x - distance_x), (position.y - distance_y),
        (position.x - distance_x), (position.y + distance_y)
        ]
    }

    fn creater_render_vertices(&self, camera: &Camera) -> Vec<f32>{
        let normalized_position: NormalizedPosition = Hexagon::world_space_to_screen_space(&self.position, camera);
        Hexagon::normalized_vertex_array(&normalized_position, camera)
    }

    pub fn render_hexagon(&self, camera: &Camera){
        self.renderer.draw_object(self.creater_render_vertices(camera));
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32){
        self.renderer.set_color(r, g, b, a);
    }
}

pub struct Camera {
    pub position: Position,
    pub scale: f64,
}

impl Camera {
    pub fn new() -> Camera{
        Camera{
            position: Position::new(),
            scale: 50.0
        }
    }

    pub fn zoom(&mut self, y: i32){
        let zoom_amount = self.scale / 100.0;
        self.scale += zoom_amount * y as f64;
    }
}

pub struct Tile {
    hexagon: Hexagon,
    hexagonal_position: CubicCoordinate,
    formula: String,
}

impl Tile {
    pub fn new(assigned_formula: String, hexagonal_coordinate: CubicCoordinate, camera: &Camera) -> Tile{
        Tile{
            hexagon: Hexagon::initialize_hexagon(Position::axial_to_position(&AxialCoordinate::cubic_to_axial(&hexagonal_coordinate)), camera),
            hexagonal_position: hexagonal_coordinate,
            formula: assigned_formula,
        }
    }

}

pub struct Chunk {
    enviromental_tiles: Vec<EnviromentalTile>,
    planetary_position: CubicCoordinate,
}

impl Chunk {
    pub fn load_chunk(camera: &Camera) -> Chunk{
        let mut tiles: Vec<EnviromentalTile> = vec![];
        let chunk_size: i128 = 50;
        for x in -chunk_size..chunk_size {
            for y in -chunk_size..chunk_size {
                for z in -chunk_size..chunk_size {
                    if x+y+z == 0 {
                        let mut tile = EnviromentalTile{tile: Tile::new("stone".to_string(),
                        CubicCoordinate{x,y,z},
                        camera)};
                        let r = (x as f32/50.0).abs();
                        let g = (y as f32/50.0).abs();
                        let b = (z as f32/50.0).abs();
                        println!("{:?}", r);
                        tile.tile.hexagon.set_color(r, g, b, 1.0);
                        tiles.push(tile);
                    }
                }
            }
        }
        Chunk{enviromental_tiles: tiles, planetary_position: CubicCoordinate::new()}
    }

    pub fn draw_tiles(&self, camera: &Camera){
        for tile in &self.enviromental_tiles {
            tile.tile.hexagon.render_hexagon(camera);
        }
    }
}

pub struct Planet {
    position: Position,
    // snap_position: OffsetCoordinate,
    planetary_position: CubicCoordinate,
    seed: *const u128,
}

pub struct EnviromentalTile {
    tile: Tile,
}

impl EnviromentalTile {

}
