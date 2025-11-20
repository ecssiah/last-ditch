use last_ditch::{interface::app::LastDitchApp, utils::tracer::Tracer};
use winit::event_loop::{ControlFlow, EventLoop};

pub async fn run() {
    #[cfg(not(feature = "profile"))]
    Tracer::new();

    #[cfg(feature = "profile")]
    let tracer = Tracer::new();

    Tracer::log_intro();

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
