use crate::{Request, Response};
use std::sync::Arc;

pub type NextFn = Box<dyn FnOnce(Request) -> Response + Send>;

pub trait Middleware: Send + Sync {
    fn handle(&self, req: Request, next: NextFn) -> Response;
}

impl<F> Middleware for F where F: Fn(Request, NextFn) -> Response + Send + Sync {
    fn handle(&self, req: Request, next: NextFn) -> Response {
        (self)(req, next)
    }
}

pub(crate) fn dispatch_middleware_chain(
    req: Request,
    middlewares: Arc<Vec<Arc<dyn Middleware>>>,
    index: usize,
    final_handler: Arc<dyn crate::Handler>,
) -> Response {
    if index < middlewares.len() {
        let current_middleware = middlewares[index].clone();
        let next_middlewares_arc = middlewares.clone();
        let next_final_handler_arc = final_handler.clone();
        let next_fn: NextFn = Box::new(move |r: Request| {
            dispatch_middleware_chain(r, next_middlewares_arc, index + 1, next_final_handler_arc)
        });

        current_middleware.handle(req, next_fn)
    } else {
        final_handler.handle(req)
    }
}