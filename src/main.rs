use actix_files as afs;
use actix_web::{
    body::BoxBody,
    dev::ServiceResponse,
    get,
    http::StatusCode,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    App, HttpResponse, HttpServer, Result as AwResult,
};

use std::io;

#[get("/")]
async fn root_handler() -> AwResult<HttpResponse> {
    let body = include_str!("../static/html/index.html");

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(body))
}

fn error_handler<B>(res: ServiceResponse<B>) -> AwResult<ErrorHandlerResponse<BoxBody>> {
    let response = afs::NamedFile::open("./static/html/error.html")
        .unwrap()
        .into_response(res.request());

    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(ErrorHandlers::new().default_handler(error_handler))
            .service(root_handler)
            .service(afs::Files::new("/", "./static").prefer_utf8(true))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
