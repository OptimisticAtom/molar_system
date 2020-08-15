mod graphics;
// mod chemistry;

pub struct Position{
    x: f64,
    y: f64,
}

impl Position {
    pub fn set_position(sx: f64, sy: f64) -> Position{
        Position{x: sx, y: sy}
    }
}

pub struct NormalizedPosition {
    x: f32,
    y: f32,
}

pub struct HexagonalPosition{
    x: i128,
    y: i128,
    z: i128
}

pub struct Hexagon{
    position: Position,
    pub renderer: graphics::Renderer,
}

impl Hexagon{
    pub fn initialize_hexagon(x: f64, y: f64, camera: &Camera) -> Hexagon{
        let pos = Position::set_position(x, y);
        let normalized = Hexagon::world_space_to_screen_space(&pos, &camera);
        let vertices = Hexagon::normalized_vertex_array(&normalized, &camera);
        let rend = graphics::Renderer::initialize_object_renderer(vertices);
        Hexagon{position: pos, renderer: rend}
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

    // fn render_hexagon(camera_scale: u32){
    //     let normalized_position: NormalizedPosition =
    // }
}

pub struct Camera {
    pub position: Position,
    pub scale: f64,
}

// pub struct Tile {
//
// }
