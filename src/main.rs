use std::time::Instant;

use opentelemetry::trace::TracerProvider;
use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use tracing::{Instrument, Level, info, span};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Initialize OTLP exporter using gRPC (Tonic)
    // 1. OTLP Exporter - The Sender
    // This is your shipping service. It packages up your tracing data and sends it to Jaeger using the OpenTelemetry Protocol (OTLP) over HTTP.
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        // for HTTP
        // .with_http()
        // .with_endpoint("http://localhost:4318/v1/traces")
        .build()?;

    // Create a tracer provider with the exporter
    // let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
    //     .with_simple_exporter(otlp_exporter)
    //     .build();

    // Factory, creates tracers, attaches metadata
    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter)
        // Service identity
        .with_resource(
            Resource::builder_empty()
                .with_attributes([
                    KeyValue::new("service.name", NAME),
                    KeyValue::new("service.version", VERSION),
                ])
                .build(),
        )
        .build();

    // Tracer - span creator
    let tracer = tracer_provider.tracer(NAME);

    // Set it as the global provider
    global::set_tracer_provider(tracer_provider);

    // // Development: Just console
    // tracing_subscriber::fmt::init();

    // // Production: Console + File + Metrics + Distributed tracing
    // tracing_subscriber::registry()
    //     .with(fmt::layer())
    //     .with(file_layer)
    //     .with(metrics_layer)
    //     .with(opentelemetry_layer)
    //     .init();

    // let console_layer = console_subscriber::ConsoleLayer::builder().spawn();

    tracing_subscriber::registry()
        .with(console_subscriber::ConsoleLayer::builder().spawn()) // tokio console
        .with(tracing_subscriber::fmt::layer()) // Layer 1: Console formatting
        .with(tracing_opentelemetry::layer().with_tracer(tracer)) // Layer 2: OTLP export
        .with(EnvFilter::from_default_env()) // Reads RUST_LOG env var
        .init();

    async {
        let start = Instant::now();

        println!("Hello!");

        let execution_time = start.elapsed();
        let nanos = execution_time.as_nanos();
        let formatted_time = match nanos {
            0..=999 => format!("{}ns", nanos),
            1_000..=999_999 => format!("{:.1}μs", nanos as f64 / 1_000.0),
            1_000_000..=999_999_999 => format!("{:.1}ms", nanos as f64 / 1_000_000.0),
            _ => format!("{:.2}s", execution_time.as_secs_f64()),
        };

        info!("Done in {}", formatted_time);
    }
    .instrument(span!(Level::INFO, "hello",))
    .await;

    Ok(())
}
