use std::io::Write;

use http::httprequest::{HttpRequest, Method, Resource};

use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) {
        match req.method {
            Method::Get => match &req.resource {
                Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => WebServiceHandler::handler(&req),
                        _ => StaticPageHandler::handler(&req),
                    }
                }
            },
            Method::Post | Method::Uninitialized => PageNotFoundHandler::handler(&req),
        }
        .send_response(stream)
        .unwrap();
    }
}
