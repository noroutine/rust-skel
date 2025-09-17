use std::future::{Ready, ready};

use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    http::header::{HeaderName, HeaderValue},
};
use futures_util::future::LocalBoxFuture;
use tracing::warn;
use tracing_actix_web::RequestId;

const REQUEST_ID_HEADER_NAME: &str = "x-request-id";

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct RequestIdHeader;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for RequestIdHeader
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestIdHeaderMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdHeaderMiddleware { service }))
    }
}

pub struct RequestIdHeaderMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestIdHeaderMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let request_id = req.extensions().get::<RequestId>().cloned();

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            if let Some(request_id) = request_id {
                match HeaderValue::try_from(request_id.to_string()) {
                    Ok(header_value) => {
                        res.headers_mut()
                            .insert(HeaderName::from_static(REQUEST_ID_HEADER_NAME), header_value);
                    }
                    Err(e) => {
                        warn!("Failed to create header value for request ID: {}", e);
                    }
                }
            } else {
                warn!("Cannot find request_id in extensions");
            }

            Ok(res)
        })
    }
}
