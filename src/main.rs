
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use openapiv3::*;
use openapiv3_codegen::{Codegen, CodegenConfig};
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use openapiv3::{OpenAPI, ReferenceOr};
use openapiv3_codegen::{Codegen, CodegenConfig, RustHyperCodegen};
use std::fs;


async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    if req.uri().path() == "/" {
        return Ok(Response::new(Body::from("checked")));
    }

    if req.uri().path() == "/ping" {
        return Ok(Response::new(Body::from("pong")));
    }

    Ok(Response::builder()
        .status(404)
        .body(Body::empty())
        .unwrap())
}

#[tokio::main]
async fn main() {
    // Load the OpenAPI YAML file
    let yaml = std::fs::read_to_string("/home/NMDo/Rust_projet/hello_cargo/API.yaml").expect("Failed to read OpenAPI file");

    // Parse the OpenAPI YAML into OpenAPI specification
    let spec: OpenAPI = serde_yaml::from_str(&yaml).expect("Failed to parse OpenAPI YAML");


    let mut config = CodegenConfig::default();
    config.base_path = Some("/".to_owned());
    config.env_vars.insert("DATABASE_URL".to_owned(), "postgres://minh:minh123@localhost:5432/".to_owned());
    let codegen = RustHyperCodegen::new(config);
    let code = codegen.generate(&spec);

    let rust_code = codegen.into_string().expect("Failed to generate Rust code");

    // Write the generated Rust code to a file
    let mut file = File::create("/home/NMDo/Rust_projet/hello_cargo/src/generated_code.rs").expect("Failed to create file");
    file.write_all(rust_code.as_bytes()).expect("Failed to write Rust code to file");

    // Create a Hyper server
    let addr = ([127, 0, 0, 1], 8000).into();
    let make_svc = make_service_fn(|_conn| {
        let yaml = yaml.clone();
        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let yaml = yaml.clone();
                handle_request(req, yaml)
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);

    // Run the server
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}