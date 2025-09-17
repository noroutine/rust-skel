use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

use tracing::{info, instrument};

use crate::requestid::RequestIdHeader;

mod setup;
mod requestid;

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
async fn main() -> anyhow::Result<()> {
    setup::tracing().await?;

    HttpServer::new(|| {
        App::new()
            .wrap(RequestIdHeader)
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
