//! Logging middleware for Actix-Web applications.
//!
//! This module provides a simple logging middleware that logs incoming requests
//! and outgoing responses to the console. It's useful for development and
//! debugging purposes.

use actix_web::{dev::{ServiceRequest, ServiceResponse, Transform, Service}, Error};
use futures::future::{ok, Ready};
use std::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;
use std::rc::Rc;

/// Middleware for logging HTTP requests and responses.
///
/// This struct implements Actix-Web's `Transform` trait to wrap services
/// with logging capabilities. It logs the HTTP method and URI of each request
/// and the status code of each response.
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
    /// The inner service being wrapped.
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

    /// Processes the request and logs both the request and response.
    ///
    /// Logs the HTTP method and URI when a request is received, then passes
    /// the request to the inner service. When the response is ready, logs
    /// the status code before returning it.
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
