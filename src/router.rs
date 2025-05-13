use std::collections::HashMap;
use std::sync::Arc;

use crate::http_method::HttpMethod;
use crate::middleware::{self, Middleware};
use crate::{Handler, Request, Response};

#[derive(Clone)]
struct RouteDefinition {
    handler: Arc<dyn Handler>,
    middlewares: Vec<Arc<dyn Middleware>>,
}

#[derive(Clone)]
pub struct Router {
    routes: HashMap<String, HashMap<HttpMethod, RouteDefinition>>,
    global_middlewares: Vec<Arc<dyn Middleware>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            global_middlewares: Vec::new(),
        }
    }

    // Adds a global middleware that will be applied to all routes.
    pub fn use_global<M>(&mut self, middleware: M) -> &mut Self
    where
        M: Middleware + 'static,
    {
        self.global_middlewares.push(Arc::new(middleware));
        self
    }

    fn add_route_internal<H>(
        &mut self,
        method: HttpMethod,
        path: &str,
        handler: H,
        route_middlewares: Vec<Arc<dyn Middleware>>,
    ) where
        H: Handler + 'static,
    {
        let route_def = RouteDefinition {
            handler: Arc::new(handler),
            middlewares: route_middlewares,
        };
        self.routes
            .entry(path.to_string())
            .or_insert_with(HashMap::new)
            .insert(method, route_def);
    }

    pub fn get<H>(&mut self, path: &str, handler: H) -> &mut Self
    where
        H: Handler + 'static,
    {
        self.add_route_internal(HttpMethod::GET, path, handler, vec![]);
        self
    }

    pub fn get_with_middlewares<H>(
        &mut self,
        path: &str,
        handler: H,
        middlewares: Vec<Arc<dyn Middleware>>
    ) -> &mut Self
    where
        H: Handler + 'static,
    {
        self.add_route_internal(HttpMethod::GET, path, handler, middlewares);
        self
    }

    pub fn post<H>(&mut self, path: &str, handler: H) -> &mut Self
    where
        H: Handler + 'static,
    {
        self.add_route_internal(HttpMethod::POST, path, handler, vec![]);
        self
    }

    pub fn post_with_middlewares<H>(
        &mut self,
        path: &str,
        handler: H,
        middlewares: Vec<Arc<dyn Middleware>>
    ) -> &mut Self
    where
        H: Handler + 'static,
    {
        self.add_route_internal(HttpMethod::POST, path, handler, middlewares);
        self
    }

    pub fn put<H>(&mut self, path: &str, handler: H) -> &mut Self
    where
        H: Handler + 'static,
    {
        self.add_route_internal(HttpMethod::PUT, path, handler, vec![]);
        self
    }

    pub fn delete<H>(&mut self, path: &str, handler: H) -> &mut Self
    where
        H: Handler + 'static,
    {
        self.add_route_internal(HttpMethod::DELETE, path, handler, vec![]);
        self
    }

    pub fn handle_request(&self, req: Request) -> Response {
        match self.routes.get(&req.path) {
            Some(methods_for_path) => {
                match methods_for_path.get(&req.method) {
                    Some(route_def) => {
                        // Combine global and route specific middlewares
                        let mut all_middlewares = self.global_middlewares.clone();
                        all_middlewares.extend(route_def.middlewares.iter().cloned());

                        middleware::dispatch_middleware_chain(
                            req,
                            Arc::new(all_middlewares),
                            0,
                            route_def.handler.clone(),
                        )
                    }
                    None => {
                        // Path exists, but not for this method
                        if req.method == HttpMethod::HEAD && methods_for_path.contains_key(&HttpMethod::GET) {
                            let get_route_def = methods_for_path.get(&HttpMethod::GET).unwrap();
                            let mut all_middlewares = self.global_middlewares.clone();
                            all_middlewares.extend(get_route_def.middlewares.iter().cloned());

                            let mut response = middleware::dispatch_middleware_chain(
                                req,
                                Arc::new(all_middlewares),
                                0,
                                get_route_def.handler.clone(),
                            );
                            response.body = String::new(); // Strip body for HEAD
                            response
                        } else {
                            Response::new(405)
                                .with_body("405 Method Not Allowed")
                        }
                    }
                }
            }
            None => Response::new(404).with_body("404 NOT FOUND")
        }
    }
}
