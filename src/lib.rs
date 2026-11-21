//! Provides a simple mechanism of routing

use http_core::Method;
use http_core::request::Request;
use http_core::response::{Response, ResponseData, StatusCode};

use std::collections::HashMap;
use std::sync::Arc;

use crate::path_parameter::path_parameters;

mod path_parameter;
mod matching;

/// Represents a target route
#[allow(missing_debug_implementations)]
pub struct Route {
    /// The target resource.
    pub target: String,
    /// The HTTP method.
    pub method: Method,
    /// The handle
    pub handle: Arc<dyn Fn(&Request, HashMap<String, String>) -> Response + Send + Sync + 'static>,
}

/// The Router
#[allow(missing_debug_implementations)]
pub struct Router {
    routes: Vec<Route>,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Router {
    /// Creates a new instance of [`Router`].
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Registers a new [`Route`] in the [`Router`].
    pub fn register_route<F>(&mut self, target: &str, method: &str, handle: F)
    where
        F: Fn(&Request, HashMap<String, String>) -> Response + 'static,
        F: Send + Sync,
    {
        self.routes.push(Route {
            target: target.to_string(),
            method: Method::to_method(method).unwrap(),
            handle: Arc::new(handle),
        });
    }

    /// Match the request againts a route.
    pub fn match_route(&self, request: &Request) -> Response {
        let target = request.request_line().target();
        let routes = self.routes.iter();
        let routes: Vec<&Route> = routes.filter(|r|  {
            matching::is_target(&r.target, target)
        }).collect();

        if routes.is_empty() {
            let mut response = Response::default();
            
            response.set_status(StatusCode::NotFound);
            response.set_body(Some(b"Not Found"));
    
            return response
        }
        
        for route in &routes {
            let params = path_parameters(&route.target, target);

            if is_route(route, request) {
                return (route.handle)(request, params);
            }
        }

        let methods: Vec<Method> = routes
            .iter()
            .map(|r| r.method)
            .collect(); 
        let mut response  = Response::default();
    
        response.set_status(StatusCode::MethodNotAllowed(methods));
        response.set_body(Some(b"Method Not Allowed"));
    
        response
    }

    /// Registers a new `GET` endpoint to the routing.
    pub fn get<T>(&mut self, uri: &str, handle: T)
    where
        T: Fn(&Request, HashMap<String, String>) -> Response + 'static + Sync + Send,
    {
        self.register_route(uri, "GET", handle);
    }

    /// Register a new `HEAD` endpoint to the router.
    pub fn head<T>(&mut self, target: &str, handle: T)
    where
        T: Fn(&Request, HashMap<String, String>) -> Response + 'static + Sync + Send,
    {
        self.register_route(target, "HEAD", handle);
    }

    pub fn post<T>(&mut self, target: &str, handle: T)
    where
        T: Fn(&Request, HashMap<String, String>) -> Response + 'static + Sync + Send,
    {
        self.register_route(target, "POST", handle);
    }

    pub fn delete<T>(&mut self, target: &str, handle: T)
    where
        T: Fn(&Request, HashMap<String, String>) -> Response + 'static + Sync + Send,
    {
        self.register_route(target, "DELETE", handle);
    }

    pub fn patch<T>(&mut self, target: &str, handle: T)
    where
        T: Fn(&Request, HashMap<String, String>) -> Response + 'static + Sync + Send,
    {
        self.register_route(target, "PATCH", handle);
    }
}

/// Checks if the route match testing `route` against a `request`.
///
/// The `switch` parameter is used for flow control where I can test:
///
/// - If both route and method match
/// - If route match, but the method not
///
/// Set `true` for the first case; Set `false` for the second case.
fn is_route(route: &Route, request: &Request) -> bool {
    let is_method = route.method == request.request_line().method();
    let is_target = matching::is_target(
        &route.target, 
        request.request_line().target()
    );

    is_method && is_target
}

#[cfg(test)]
mod router {
    use super::*;
    
    use http_core::request::RequestLine;
    
    #[test]
    fn router_match() {
        let req_line = RequestLine::build(
            http_core::request::HttpVersion::V1, 
            Method::GET, 
            "/test"
        );
        let request = Request::build(req_line, None, None);
        let mut router = Router::new();

        router.get("/test", |_, _| Response::default());

        let res = router.match_route(&request);
        assert_eq!(res.status().code(), "200");
    }

    #[test]
    fn router_not_found() {
        let req_line = RequestLine::build(
            http_core::request::HttpVersion::V1, 
            Method::GET, 
            "/ops"
        );
        let request = Request::build(req_line, None, None);
        let mut router = Router::new();

        router.get("/test", |_, _| Response::default());

        let res = router.match_route(&request);
        assert_eq!(res.status().code(), "404");
    }

    #[test]
    fn router_with_multiples_methods_works() {
        let req_line = RequestLine::build(
            http_core::request::HttpVersion::V1, 
            Method::HEAD, 
            "/test"
        );
        let request = Request::build(req_line, None, None);
        let mut router = Router::new();

        router.get("/test", |_, _| Response::default());
        router.head("/test", |_,_| Response::default());

        let res = router.match_route(&request);
        assert_eq!(res.status().code(), "200");
    }

    #[test]
    fn params_match() {
        let req_line = RequestLine::build(
            http_core::request::HttpVersion::V1, 
            Method::GET, 
            "/test/50"
        );
        
        let request = Request::build(req_line, None, None);
        let mut router = Router::new();

        router.get("/test/:id", |_, params| {
            let params = params;
            assert_eq!(params.get("id").unwrap(), "50");
            
            Response::default()
        });

        let res = router.match_route(&request);
        assert_eq!(res.status().code(), "200");
    }
}