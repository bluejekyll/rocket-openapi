//! Create an OpenAPI spec from Rocket.


#![allow(missing_docs)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate log;
extern crate openapi;
extern crate rocket;
extern crate rocket_contrib;

use std::collections::BTreeMap;

use openapi::*;
use rocket::http::Method;
use rocket::request::State;
use rocket::Rocket;
use rocket_contrib::JSON;

/// An endpoint for the OpenAPI Spec
#[get("/spec", format = "application/json")]
pub fn spec<'s>(openapi: State<'s, Spec>) -> JSON<&'s Spec> {
    JSON(openapi.inner())
}

/// Builds an OpenAPI Spec from a lit `Rocket`.
///
/// # Arguments
///
/// * `lit` - fully configured `Rocket`
/// * `service_name` - the human name of the service
/// * `service_version` - the current version of the service
pub fn build_swagger_spec(lit: &Rocket, service_name: &str, service_version: &str) -> Spec {
    let mut paths = BTreeMap::<String, Operations>::new();
    for route in lit.routes() {
        debug!("documenting route: {}", route);
        let path = route.uri.path().to_string().replace("<", "{").replace(">", "}");
        let mut ops = paths.entry(path).or_insert(Operations::default());

        let param_indexes: Vec<(usize, usize)> = route.get_param_indexes(&route.uri);
        let url_path = route.uri.as_str();

        let parameters = param_indexes.into_iter()
            .map(|index| {
                &url_path[{
                    index.0 + 1
                }..
                 {
                    index.1 - 1
                }] // strip '<' and '>'
            })
            .map(|rckt_param| {
                ParameterOrRef::Parameter {
                    name: rckt_param.to_string(),
                    location: "path".to_string(),
                    required: Some(true),
                    param_type: Some("string".to_string()),
                    description: None,
                    format: None,
                    schema: None,
                    unique_items: None,
                }
            })
            .collect::<Vec<_>>();

        let mut op = Operation::default();
        // TODO: extract first line of docs...
        // op.summary = Some(format!("{}", route));
        op.responses = BTreeMap::new();
        op.parameters = Some(parameters);

        match route.method {
            Method::Connect => warn!("Connect not supported yet"),
            Method::Delete => ops.delete = Some(op),
            Method::Get => ops.get = Some(op),
            Method::Head => ops.head = Some(op),
            Method::Options => ops.options = Some(op),
            Method::Patch => ops.patch = Some(op),
            Method::Post => ops.post = Some(op),
            Method::Put => ops.put = Some(op),
            Method::Trace => warn!("Trace not supported yet"),
        }
    }

    let mut spec = Spec::default();
    spec.swagger = "2.0".to_string();
    spec.info = Info {
        title: Some(service_name.to_string()),
        version: Some(service_version.to_string()),
        ..Info::default()
    };
    spec.paths = paths;

    spec
}
