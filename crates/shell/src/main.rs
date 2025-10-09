use anyhow::Result;
use egui::{CtxRef, CentralPanel, TextEdit};
use egui_winit::winit;
use egui_winit::winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<()> {
    // Set up window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Crynn Browser")
        .with_inner_size(winit::dpi::LogicalSize::new(1000.0, 700.0))
        .build(&event_loop)?;

    // Create egui context
    let mut egui_ctx = egui::Context::default();
    let mut state = egui_winit::State::new(&event_loop);

    // State: URL bar string
    let mut url_input = String::from("https://valetiportfolio.vercel.app/");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match &event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }

        // Pass events to egui
        let _ = state.on_event(&egui_ctx, &event);

        if let Event::RedrawRequested(_) = event {
            let raw_input = state.take_egui_input(&window);
            egui_ctx.begin_frame(raw_input);

            // UI layout
            CentralPanel::default().show(&egui_ctx, |ui| {
                ui.heading("Crynn Browser");
                ui.horizontal(|ui| {
                    ui.label("URL:");
                    ui.add(TextEdit::singleline(&mut url_input).desired_width(600.0));
                    if ui.button("Go").clicked() {
                        println!("TODO: navigate to {url_input}");
                    }
                });
            });

            let (_output, shapes) = egui_ctx.end_frame();
            let clipped_meshes = egui_ctx.tessellate(shapes);
            let mut frame = egui_winit::winit::platform::run_return::Frame::new(&window);
            egui_winit::egui_glium::paint(&window, &mut frame, &clipped_meshes);
        }

        if let Event::MainEventsCleared = event {
            window.request_redraw();
        }
    });
}
