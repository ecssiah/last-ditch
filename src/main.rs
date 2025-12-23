use last_ditch::{interface::app::App, utils::tracer::Tracer};
use std::sync::OnceLock;
use winit::event_loop::{ControlFlow, EventLoop};

static TRACER: OnceLock<Tracer> = OnceLock::new();

fn main() {
    let tracer = Tracer::new();
    TRACER.set(tracer).unwrap();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();

    #[cfg(feature = "profile")]
    if let Some(tracer) = TRACER.get() {
        Tracer::export(&tracer.flamegraph_name);
    }
}
