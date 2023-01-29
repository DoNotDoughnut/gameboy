use std::time::Duration;

use instant::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod gb;

// mod file;

fn main() {

    let rom = std::fs::read("test.bin").expect("Could not open test ROM!");

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Gameboy Emulator")
        .build(&event_loop)
        .unwrap();

    let size = window.inner_size();

    let surface = pixels::SurfaceTexture::new(size.width, size.height, &window);

    let mut pixels = pollster::block_on(pixels::Pixels::new_async(160, 144, surface))
        .expect("Could not create window!");

    // simple_logger::init_with_level(log::Level::Info);

    // bevy_tasks::IoTaskPool::init(|| bevy_tasks::TaskPool::default());
    // bevy_tasks::ComputeTaskPool::init(|| bevy_tasks::TaskPool::default());
    // bevy_tasks::AsyncComputeTaskPool::init(|| bevy_tasks::TaskPool::default());

    // #[cfg(not(target_arch = "wasm32"))]
    // let thread = std::thread::spawn(|| {
    //     loop {
    //         std::thread::sleep(Duration::from_millis(3));
    //         bevy_tasks::tick_global_task_pools_on_main_thread();
    //     }
    // });

    // let mut file_handler = FileHandler::new();

    // file_handler.load();

    let mut emulator = gb::GameboyColor::new();

    emulator.set_cartridge(&rom).unwrap();

    const CYCLE_TIME: Duration = Duration::new(0, 16600000);
    const CLOCK_SPEED: usize = 4194304;
    const FRAME_RATE: usize = 60;
    const CYCLES_PER_FRAME: usize = CLOCK_SPEED / FRAME_RATE;

    // to do make better event loop

    let mut next = Instant::now();

    event_loop.run(move |event, _target, flow| {
        let mut step = None;

        let new = Instant::now();
        if new >= next {
            let between = new - next;
            next += CYCLE_TIME;
            *flow = ControlFlow::WaitUntil(next);
            step = Some(between);
        }

        if let Some(between) = step {
            let mut cycles: usize = 0;
            while cycles <= (CLOCK_SPEED as f64 * between.as_secs_f64()) as usize {
                cycles += emulator.step();
            }

            window.request_redraw();
        }

        // if file_handler.alive() {
        //     if file_handler.tick() {
        //         emulator.set_rom(&file_handler.take());
        //     }
        // }

        match event {
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::CloseRequested => {
                    if window_id == window.id() {
                        *flow = ControlFlow::Exit;
                    }
                }
                WindowEvent::Resized(size) => {
                    pixels.resize_surface(size.width, size.height).unwrap();
                }
                _ => (),
            },
            Event::RedrawRequested(wid) => {
                if wid == window.id() {
                    // pixels.get_frame_mut()
                    emulator.render();
                }
            }
            Event::LoopDestroyed => {
                //#[cfg(not(target_arch = "wasm32"))]
                // kill thread here
            }
            _ => (),
        }
    })
}
