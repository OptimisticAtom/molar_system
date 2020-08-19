

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
    let mut event_pump = sdl.event_pump().unwrap();
    let mut game = game_state::Simulation::start_simulation(60);
    'main: loop{
        for event in event_pump.poll_iter() {
            match event{
                sdl2::event::Event::Quit {..} => break 'main, _ => {},
                // sdl2::event::Event::MouseWheel {..} => game.main_camera.zoom(event.y),
            }
        }
        game.loop_call();
        window.gl_swap_window();
    }
}
