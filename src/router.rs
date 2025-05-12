use std::collections::HashMap;
use std::sync::Arc;

use crate::{Handler, Request, Response};

#[derive(Clone)]
pub struct Router {
    routes: HashMap<String, Arc<dyn Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: Handler + 'static,
    {
        self.routes.insert(path.to_string(), Arc::new(handler));
    }

    pub fn handle_request(&self, req: Request) -> Response {
        if let Some(handler) = self.routes.get(&req.path) {
            handler.handle(req)
        } else {
            Response::new(404).with_body("404 NOT FOUND")
        }
    }
}
