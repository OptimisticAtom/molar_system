

extern crate sdl2;
extern crate gl;


// mod graphics;
// mod game_objects;
mod game_state;


fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4,5);
    let window = video_subsystem.window("Molar System", 1000, 1000).opengl().resizable().build().unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const std::os::raw::c_void);
    unsafe{
        gl::Viewport(0,0,1000,1000);
        gl::ClearColor(0.05,0.0,0.15,1.0);
        // gl::MatrixMode(gl::GL_PROJECTION);
        // gl::Enable(gl::GL_BLEND);
        // gl::BlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

    }
    // let vao = graphics::do_graphics_stuff();
    // let hex = graphics::Renderer::initialize_object_renderer();
    let mut event_pump = sdl.event_pump().unwrap();
    // let mut r = 0.1;
    // let mut dr = 0.05;
    // let s = "u_Color";
    // let camera_position = game_objects::Position::new();
    // let camera = game_objects::Camera{position: camera_position, scale: 100.0};
    // let mut hex1 = game_objects::Hexagon::initialize_hexagon(-50.0, 50.0, &camera);
    // let mut hex2 = game_objects::Hexagon::initialize_hexagon(50.0, -50.0, &camera);
    let game = game_state::Simulation::start_simulation();
    'main: loop{
        for event in event_pump.poll_iter() {
            match event{
                sdl2::event::Event::Quit {..} => break 'main, _ => {},
            }
        }
        // if r >= 0.95{dr = -0.05} else if r <= 0.05 {dr = 0.05}
        // r += dr;
        // let mut location = unsafe {gl::GetUniformLocation(vao[2], s.as_ptr() as *const gl::types::GLbyte)};
        // graphics::draw(vao, location, r);
        // hex.draw_object();
        // hex1.position.x += 0.01;
        // hex2.position.x -= 0.1;
        unsafe{gl::Clear(gl::COLOR_BUFFER_BIT);}
        game.chunck_loader.draw_chunks(&game.main_camera);
        // hex1.render_hexagon(&camera);
        // hex2.render_hexagon(&camera);
        window.gl_swap_window();
    }
}
