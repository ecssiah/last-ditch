use last_ditch::{interface::app::LastDitchApp, utils::tracer::Tracer};
use std::sync::OnceLock;
use winit::event_loop::{ControlFlow, EventLoop};

static TRACER: OnceLock<Tracer> = OnceLock::new();

pub async fn run() {
    let tracer = Tracer::new();
    TRACER.set(tracer).unwrap();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut last_ditch_app = LastDitchApp::default();
    event_loop.run_app(&mut last_ditch_app).unwrap();

    #[cfg(feature = "profile")]
    Tracer::export(&tracer.flamegraph_name);
}

#[tokio::main]
async fn main() {
    run().await;
}
