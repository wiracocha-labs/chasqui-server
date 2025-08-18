use actix_web::{dev::{ServiceRequest, ServiceResponse, Transform, Service}, Error};
use futures::future::{ok, Ready};
use std::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;
use std::rc::Rc;

// Logging struct represents the logging middleware
pub struct Logging;

// Implementation of the Transform trait for Logging
impl<S, B> Transform<S, ServiceRequest> for Logging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // Create a new instance of LoggingMiddleware
    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware {
            service: Rc::new(service),
        })
    }
}

// LoggingMiddleware struct holds the wrapped service
pub struct LoggingMiddleware<S> {
    service: Rc<S>,
}

// Implementation of the Service trait for LoggingMiddleware
impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    // Check if the service is ready to handle a request
    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    // Handle the incoming request
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Log the incoming request
        println!("Request: {} {}", req.method(), req.uri());
        
        // Call the wrapped service
        let fut = self.service.call(req);
        
        // Return a future that will log the response
        Box::pin(async move {
            let res = fut.await?;
            // Log the outgoing response
            println!("Response: {}", res.status());
            Ok(res)
        })
    }
}
