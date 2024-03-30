use actix_web::{
    get, middleware::Logger, post, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

use std::error::Error;
use std::path::Path;

use reqwest::{self};

#[derive(Deserialize, Serialize, Debug)]
struct User {
    routes: Vec<Route>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Route {
    host: String,
}

// Handler para rota de exemplo
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    let name = req.match_info().get("name").unwrap_or("World");
    Ok(HttpResponse::Ok().body(format!("Hello {}!", &name)))
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

async fn fetch_get_url(url: &str) -> Result<std::string::String, actix_web::Error> {
    let client = reqwest::Client::new();
    let resp = match client.get(url).send().await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(err) => {
            panic!("Error: {}", err)
        }
    };
    Ok(resp)
}

fn remove_quotes(s: &str) -> String {
    s.trim_matches(|c| c == '\"' || c == '\'').to_string()
}

#[get("/{tail:.*}")]
async fn get_request(req: HttpRequest) -> String {
    let routes = read_user_from_file("routes.json").unwrap();
    let mounted = serde_json::to_string(&routes).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&mounted).unwrap();

    let mounted_url = remove_quotes(&parsed["routes"][0]["host"].to_string());
    // println!("{}", mounted_url);
    let response_fetch = fetch_get_url(mounted_url.as_str()).await;
    response_fetch.unwrap()
}

#[get("/configuration")]
async fn configuration() -> String {
    let routes = read_user_from_file("routes.json").unwrap();
    let mounted = serde_json::to_string(&routes).unwrap();
    return mounted;
}

#[post("/{tail:.*}")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // .wrap(Logger::new("%a %{User-Agent}i"))
            .service(echo)
            .service(configuration)
            .service(get_request)
        // .route("/hey", web::get().to(manual_hello))
        // .service(
        //     web::resource("/{tail:.*}")
        //         //.get(|| HttpResponse::Ok()),
        //         .route(web::get().to(|req: HttpRequest| {
        //             let path = req.match_info().query("tail");
        //             println!("{}", path);
        //             println!("req {}", req.uri());
        //             // println!("{:#?}", routesnew);
        //             // index_handler(req, routesnew.clone());
        //             let routesnew =
        //                 Arc::new(read_user_from_file("routes.json").unwrap().routes);
        //             for route in routesnew.iter() {
        //                 println!("{:#?}", route);
        //             }
        //             //     // Clone the host for the closure
        //             //     let route_host = route.host.clone();
        //             //     // Create an Arc for the host to ensure it lives long enough
        //             //     let route_host_arc = Arc::new(route_host);
        //             //     // Create a clone for the async block
        //             //     let uri = format!("http://{}", route_host_arc);
        //             //     let uri_clone = uri.clone();
        //             //     // Spawn async block with a new Arc reference
        //             //     let handle = tokio::spawn(async move {
        //             //         let uri = uri_clone.parse::<Uri>().expect("Failed to parse URI");
        //             //         if let Err(err) = fetch_url(uri).await {
        //             //             eprintln!("Failed to fetch URL {}: {}", route_host_arc, err);
        //             //         }
        //             //     });
        //             //     // Ensure the handle doesn't drop until the end of the loop iteration
        //             //     handle
        //             // }
        //             // for route in &routes.routes {
        //             //     fetch_url(route).await;
        //             // }
        //             // Ok(format!("Welcome {}!", info.username))
        //             HttpResponse::Ok()
        //             // match path_tree.find(&path) {
        //             //     Some(handler) => handler(req),
        //             //     None => HttpResponse::NotFound().body("Not Found"),
        //             // }
        //         })),
        // )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
