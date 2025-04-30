use crate::{Request, Response};

pub trait Handler: Send + Sync {
    fn handle(&self, req: Request) -> Response;
}

impl<F> Handler for F where F: Fn(Request) -> Response + Send + Sync, {
    fn handle(&self, req: Request) -> Response {
        (self)(req)
    }
}