mod graphics;
mod chemistry;

pub struct Position{
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new() -> Position{
        Position{x: 0.0, y: 0.0}
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
}

pub struct AxialCoordinate{
    x: i16,
    y: i16,
}

impl AxialCoordinate {
    pub fn new() -> AxialCoordinate{
        AxialCoordinate{x: 0, y: 0}
    }
}

pub struct OffsetCoordinate{
    x: i128,
    y: i128,
}

impl OffsetCoordinate {
    pub fn new() -> OffsetCoordinate{
        OffsetCoordinate{x: 0, y: 0}
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

    pub fn initialize_hexagon(x: f64, y: f64, camera: &Camera) -> Hexagon{
        let mut hexagon = Hexagon::new();
        hexagon.position.x = x;
        hexagon.position.y = y;
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
        let distance_x = 0.8660254038 / scale;
        let distance_y = 0.5 / scale;
        vec![
        position.x, (position.y + (1.0 / scale)),
        (position.x + distance_x), (position.y + distance_y),
        (position.x + distance_x), (position.y - distance_y),
        position.x, (position.y - (1.0 / scale)),
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
}

pub struct Camera {
    pub position: Position,
    pub scale: f64,
}

impl Camera {
    pub fn new() -> Camera{
        Camera{
            position: Position::new(),
            scale: 100.0
        }
    }
}

pub struct Tile {
    hexagon: Hexagon,
    formula: String,
}

impl Tile {
    pub fn new(assigned_formula: String, x: f64, y: f64, camera: &Camera) -> Tile{
        Tile{
            hexagon: Hexagon::initialize_hexagon(x, y, camera),
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
        // let tiles = vec![vec![AxialCoordinate::new()]];
        // let l = 1;
        let mut tiles: Vec<EnviromentalTile> = vec![];
        for i in -100..101 {
            // if i > 0{
            //     l = 6*i
            // }
            // for j in [0..l + 1] {
            //
            // }
            tiles.push(EnviromentalTile{tile: Tile::new("stone".to_string(), i as f64, i as f64, camera)});
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
