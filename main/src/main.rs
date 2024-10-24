use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Triangle!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
        .unwrap();

    let context = window.gl();


    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );

    let positions = vec![
        vec3(0.5, -0.5, 0.0), // Bottom right
        vec3(-0.5, -0.5, 0.0), // Bottom left
        vec3(0.0, 0.5, 0.0), // top
    ];
    let colors = vec![
        Srgba::RED, // bottom right
        Srgba::GREEN, // bottom left
        Srgba::BLUE, // top
    ];
    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    };

    let mut model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());
    model.set_animation(|time| Mat4::from_angle_y(radians(time * 0.005)));

    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);
        model.animate(frame_input.accumulated_time as f32);
        // Get the screen render target to be able to render something on the screen.
        frame_input.screen()
            // Clear the color and depth of the screen render target.
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            // Render the triangle with the color material which uses the per vertex colors defined at construction.
            .render(&camera, &model, &[]);

        // Returns default frame output to end the frame.
        FrameOutput::default()
    });
}


// use three_d::Key::F;
// use winit::event_loop::ControlFlow;
// use winit_test::App;
// use winit::window::

// pub fn main() {
//    let event_loop = winit::event_loop::EventLoop::new().unwrap();
//     event_loop.set_control_flow(ControlFlow::Poll);
//
//     let mut app = App::default();
//     event_loop.run_app(&mut app).unwrap();
//
//     let mut window = app.window.unwrap();
//     let mut wind = three_d::Window::new(WindowSettings::default()).unwrap();
//     let context = wind.gl();
//     context.
//     window.set_title("Test Window");
//     window.set_min_inner_size(Some(winit::dpi::LogicalSize::new(1280, 720)));
//     let context = WindowedContext::from_winit_window(&window)
// }
