

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
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

    }
    let mut event_pump = sdl.event_pump().unwrap();
    let mut game = game_state::Simulation::new(60);
    game.start_simulation();
    // let mut keydown = false;
    'main: loop{
        for event in event_pump.poll_iter() {
            match event{
                sdl2::event::Event::Quit {..} => break 'main,
                // sdl2::event::Event::AppTerminating { timestamp } => {},
                // sdl2::event::Event::AppLowMemory { timestamp } => {},
                // sdl2::event::Event::AppWillEnterBackground { timestamp } => {},
                // sdl2::event::Event::AppDidEnterBackground { timestamp } => {},
                // sdl2::event::Event::AppWillEnterForeground { timestamp } => {},
                // sdl2::event::Event::AppDidEnterForeground { timestamp } => {},
                // sdl2::event::Event::Window { timestamp, window_id, win_event } => {},
                // sdl2::event::Event::KeyDown { timestamp: _, window_id: _, keycode: _, scancode: _, keymod: _, repeat: _ } => {keydown = true},
                // sdl2::event::Event::KeyUp { timestamp, window_id, keycode, scancode, keymod, repeat } => {},
                // sdl2::event::Event::TextEditing { timestamp, window_id, text, start, length } => {},
                // sdl2::event::Event::TextInput { timestamp, window_id, text } => {},
                // sdl2::event::Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => {},
                // sdl2::event::Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {},
                // sdl2::event::Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } => {},
                sdl2::event::Event::MouseWheel { timestamp: _, window_id: _, which: _, x: _, y, direction: _} => game.main_camera.zoom(y),
                // sdl2::event::Event::JoyAxisMotion { timestamp, which, axis_idx, value } => {},
                // sdl2::event::Event::JoyBallMotion { timestamp, which, ball_idx, xrel, yrel } => {},
                // sdl2::event::Event::JoyHatMotion { timestamp, which, hat_idx, state } => {},
                // sdl2::event::Event::JoyButtonDown { timestamp, which, button_idx } => {},
                // sdl2::event::Event::JoyButtonUp { timestamp, which, button_idx } => {},
                // sdl2::event::Event::JoyDeviceAdded { timestamp, which } => {},
                // sdl2::event::Event::JoyDeviceRemoved { timestamp, which } => {},
                // sdl2::event::Event::ControllerAxisMotion { timestamp, which, axis, value } => {},
                // sdl2::event::Event::ControllerButtonDown { timestamp, which, button } => {},
                // sdl2::event::Event::ControllerButtonUp { timestamp, which, button } => {},
                // sdl2::event::Event::ControllerDeviceAdded { timestamp, which } => {},
                // sdl2::event::Event::ControllerDeviceRemoved { timestamp, which } => {},
                // sdl2::event::Event::ControllerDeviceRemapped { timestamp, which } => {},
                // sdl2::event::Event::FingerDown { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => {},
                // sdl2::event::Event::FingerUp { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => {},
                // sdl2::event::Event::FingerMotion { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => {},
                // sdl2::event::Event::DollarGesture { timestamp, touch_id, gesture_id, num_fingers, error, x, y } => {},
                // sdl2::event::Event::DollarRecord { timestamp, touch_id, gesture_id, num_fingers, error, x, y } => {},
                // sdl2::event::Event::MultiGesture { timestamp, touch_id, d_theta, d_dist, x, y, num_fingers } => {},
                // sdl2::event::Event::ClipboardUpdate { timestamp } => {},
                // sdl2::event::Event::DropFile { timestamp, window_id, filename } => {},
                // sdl2::event::Event::DropText { timestamp, window_id, filename } => {},
                // sdl2::event::Event::DropBegin { timestamp, window_id } => {},
                // sdl2::event::Event::DropComplete { timestamp, window_id } => {},
                // sdl2::event::Event::AudioDeviceAdded { timestamp, which, iscapture } => {},
                // sdl2::event::Event::AudioDeviceRemoved { timestamp, which, iscapture } => {},
                // sdl2::event::Event::RenderTargetsReset { timestamp } => {},
                // sdl2::event::Event::RenderDeviceReset { timestamp } => {},
                // sdl2::event::Event::User { timestamp, window_id, type_, code, data1, data2 } => {},
                // sdl2::event::Event::Unknown { timestamp, type_ } => {},
                _ => {}
            }
        }
        game.handle_keyboard_event(&mut event_pump);
        game.loop_call();
        window.gl_swap_window();
    }
}
