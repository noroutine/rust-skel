use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

use opentelemetry::trace::TracerProvider;
use opentelemetry::{KeyValue, global};
use opentelemetry_sdk::Resource;
use tracing::{info, instrument};

use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[get("/")]
#[instrument]
async fn root() -> impl Responder {
    info!("Hello world!");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Initialize OTLP exporter using gRPC (Tonic)
    // 1. OTLP Exporter - The Sender
    // This is your shipping service. It packages up your tracing data and sends it to Jaeger using the OpenTelemetry Protocol (OTLP) over HTTP.
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()?;

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

    tracing_subscriber::registry()
        .with(console_subscriber::ConsoleLayer::builder().spawn()) // tokio console
        .with(
            tracing_subscriber::fmt::layer()
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE),
        ) // Layer 1: Console formatting
        .with(tracing_opentelemetry::layer().with_tracer(tracer)) // Layer 2: OTLP export
        .with({
            let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
            // filter.add_directive("tower_http=debug".parse().unwrap())
            filter
        })
        .init();

    HttpServer::new(|| {
        App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(root)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("::", 3000))?
    .run()
    .await?;

    Ok(())
}
