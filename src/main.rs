extern crate actix_web;
extern crate mongo_driver;

use actix_web::{server, App, HttpRequest, HttpResponse, http::Method, http::StatusCode};
use std::sync::Arc;
use mongo_driver::client::{ClientPool, Uri};

fn index(_req: &HttpRequest) -> HttpResponse {
   HttpResponse::with_body(StatusCode::OK, "Hello World!")
}

fn main() {
    let connection_url = "mongodb://dev-admin:JDqSGy23sgWQYolq@development-shard-00-00-tvmgi.mongodb.net:27017,development-shard-00-01-tvmgi.mongodb.net:27017,development-shard-00-02-tvmgi.mongodb.net:27017/test?ssl=true&replicaSet=development-shard-0&authSource=admin&retryWrites=true"
    let uri = Uri::new(connection_url).unwrap();
    let pool = Arc::new(ClientPool::new(uri.clone(), None));
    let client = pool.pop();
    println!(client.get_server_status(None).unwrap());

    server::new(|| App::new()
        .prefix("/test-app")
        .resource("/", |r| r.method(Method::GET).f(index)))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
}
