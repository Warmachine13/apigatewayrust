use actix_web::{
    get, middleware::Logger, post, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

use crate::infra;

fn remove_quotes(s: &str) -> String {
    s.trim_matches(|c| c == '\"' || c == '\'').to_string()
}

#[get("/{tail:.*}")]
async fn get(req: HttpRequest) -> String {
    let path = req.match_info().query("tail");
    println!("{}", path);
    println!("req {}", req.uri());
    // let headers = req.headers().clone().();
    // println!("{}", &headers);
    // let routes = read_user_from_file("routes.json").unwrap();
    // let mounted = serde_json::to_string(&routes).unwrap();
    // let parsed: serde_json::Value = serde_json::from_str(&mounted).unwrap();

    // let mounted_url = remove_quotes(&parsed["routes"][0]["host"].to_string());
    let response_fetch = infra::rest::get("http://127.0.0.1:8090").await;
    // println!("reponse aa{}", response_fetch.unwrap());
    // "".to_string()
    response_fetch.unwrap()
}

#[post("/{tail:.*}")]
async fn post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
