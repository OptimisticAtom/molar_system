extern crate sdl2;
extern crate gl;


mod graphics;


fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4,5);
    let window = video_subsystem.window("Molar System", 900, 900).opengl().resizable().build().unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const std::os::raw::c_void);
    unsafe{
        gl::Viewport(0,0,900,900);
        gl::ClearColor(0.05,0.0,0.15,1.0);
    }
    let vao = graphics::do_graphics_stuff();
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop{
        for event in event_pump.poll_iter() {
            match event{
                sdl2::event::Event::Quit {..} => break 'main, _ => {},
            }
        }
        graphics::draw(vao);
        window.gl_swap_window();
    }
}
