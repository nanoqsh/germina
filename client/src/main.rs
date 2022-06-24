mod scheduler;

use crate::scheduler::Scheduler;
use engine::Engine;
use render::render::Render;

fn main() -> ! {
    env_logger::init();
    start()
}

fn start() -> ! {
    use winit::{
        dpi::PhysicalSize,
        event::*,
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    const WINDOW_SIZE: (u32, u32) = (800, 600);
    const WINDOW_TITLE: &str = "Germina";

    let scheduler = Scheduler::new();
    // Example of tasks creation
    for i in 0..10 {
        scheduler.spawn(async move { i * i });
    }

    let (window, el) = {
        let el = EventLoop::new();
        let window = Box::new(
            WindowBuilder::new()
                .with_inner_size({
                    let (width, height) = WINDOW_SIZE;
                    PhysicalSize::new(width, height)
                })
                .with_title(WINDOW_TITLE)
                .build(&el)
                .expect("build the window"),
        );
        let window: &'static _ = Box::leak(window);
        (window, el)
    };

    let mut engine = {
        let render = async_std::task::block_on(Render::new(window));
        let mut engine = Engine::new(render);
        engine.resize_view(WINDOW_SIZE);
        engine
    };

    el.run(move |ev, _, flow| match ev {
        Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
            WindowEvent::Resized(size)
            | WindowEvent::ScaleFactorChanged {
                new_inner_size: &mut size,
                ..
            } => engine.resize_view(size.into()),
            _ => {}
        },
        Event::MainEventsCleared => {
            // Render
            engine.draw_view();

            // Process reports of ready tasks
            for report in scheduler.ready() {
                let id = report.id;
                let value: &i32 = report.value.downcast_ref().unwrap();
                println!("{id:?}: {value}");
            }
        }
        _ => {}
    })
}
