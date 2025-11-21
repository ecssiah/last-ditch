use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
#[cfg(feature = "profile")]
use {
    chrono::Local,
    inferno::flamegraph::{from_reader, Options},
    std::fs::{self, File},
    tracing_flame::{FlameLayer, FlushGuard},
};

#[derive(Debug)]
pub struct Tracer {
    pub engine_guard: WorkerGuard,
    #[cfg(feature = "profile")]
    pub flame_guard: FlushGuard<std::io::BufWriter<std::fs::File>>,
    #[cfg(feature = "profile")]
    pub flamegraph_name: String,
}

impl Tracer {
    pub fn new() -> Self {
        let engine_file = tracing_appender::rolling::daily("logs/engine", "engine.log");
        let (engine_writer, engine_guard) = tracing_appender::non_blocking(engine_file);

        let engine_events = tracing_subscriber::fmt::layer()
            .with_writer(engine_writer)
            .with_ansi(false);

        #[cfg(feature = "profile")]
        let flamegraph_name = Self::get_flamegraph_name();

        #[cfg(feature = "profile")]
        let (flame_layer, flame_guard) =
            FlameLayer::with_file(format!("logs/traces/{}.folded", flamegraph_name).clone())
                .unwrap();

        #[cfg(not(feature = "profile"))]
        let flame_layer = tracing_subscriber::layer::Identity::new();

        tracing_subscriber::registry()
            .with(EnvFilter::new("info"))
            .with(flame_layer)
            .with(engine_events)
            .try_init()
            .expect("global subscriber already initialized!");

        Self::log_intro();

        Self {
            engine_guard,
            #[cfg(feature = "profile")]
            flame_guard,
            #[cfg(feature = "profile")]
            flamegraph_name,
        }
    }

    pub fn log_intro() {
        let decoration1 = " =  =  =  =   =  =  =  = ";
        let decoration2 = "= = = = = = = = = = = = =";
        let title = "   L A S T   D I T C H   ";
        let company = "     J U S T   S K Y     ";
        let repo = "https://github.com/ecssiah/last-ditch";

        info!(
            "\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\nVersion: {}\nRepo: {}\n\n",
            decoration1,
            decoration2,
            title,
            company,
            decoration2,
            decoration1,
            env!("CARGO_PKG_VERSION"),
            repo
        );
    }

    #[cfg(feature = "profile")]
    fn get_flamegraph_name() -> String {
        let date = Local::now().format("%Y-%m-%d").to_string();

        fs::create_dir_all("logs/traces").ok();

        let mut count = 1;

        if let Ok(entries) = fs::read_dir("logs/traces") {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with(&format!("profile.{date}-")) {
                        if let Some(part) = name.split('-').last() {
                            if let Some(num) = part.split('.').next() {
                                if let Ok(n) = num.parse::<u32>() {
                                    count = count.max(n + 1);
                                }
                            }
                        }
                    }
                }
            }
        }

        format!("profile.{date}-{count:04}")
    }

    #[cfg(feature = "profile")]
    pub fn export(flamegraph_name: &str) {
        let mut options = Options::default();

        let folded_path = format!("logs/traces/{}.folded", flamegraph_name);
        let svg_path = format!("logs/traces/{}.svg", flamegraph_name);

        let folded_graph = File::open(&folded_path).unwrap();
        let mut svg_graph = File::create(&svg_path).unwrap();

        from_reader(&mut options, folded_graph, &mut svg_graph).unwrap();

        let _ = std::fs::remove_file(&folded_path);
    }
}
