use actix_web::http::Uri;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use hyper_util::rt::TokioIo;
use path_tree::PathTree;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tokio::stream;

use std::error::Error;
use std::path::Path;

// use hyper::{body::HttpBody as _, Client, Uri};
// use std::error::Error;

// mod support;
// use support::TokioIo;

// #[path = "../benches/support/mod.rs"]
// mod support;
// use support::TokioIo;

use http_body_util::{BodyExt, Empty};
use hyper::Request;
// use hyper::{body::HttpBody as _, Client, Uri};
use tokio::io::{self, AsyncWriteExt as _};
use tokio::net::TcpStream;

// mod support;
// use support::TokioIo;

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Deserialize, Serialize, Debug)]
struct User {
    routes: Vec<Route>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Route {
    host: String,
}

// Estrutura para representar uma rota
// #[derive(Debug, Serialize, Deserialize)]
// struct Route {
//     path: String,
//     handler: String,
// }

// Handler para rota de exemplo
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    // Extrai parâmetros da URL
    let name = req.match_info().get("name").unwrap_or("World");
    Ok(HttpResponse::Ok().body(format!("Hello {}!", &name)))
}

// Handler para rota de exemplo com query parameters
// async fn echo_query(req: HttpRequest) -> Result<HttpResponse> {
//     // Extrai parâmetros da query string
//     let params: HashMap<String, String> = req.query_string().parse().unwrap();
//     Ok(HttpResponse::Ok().json(params))
// }

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

// async fn fetch_url(url: hyper::Uri) -> Result<()> {}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    println!("entrou fetch");
    println!("{}", url);
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    let request = Request::builder()
        .method("GET")
        .uri(addr)
        .header("X-Custom-Foo", "Bar")
        .body(())
        .unwrap();
    println!("{:?}", request);
    // let host = url.host().expect("uri has no host");
    // let port = url.port_u16().unwrap_or(80);
    // let addr = format!("{}:{}", host, port);
    // let stream = TcpStream::connect(addr).await?;
    // let io = Client::new(stream);

    // let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    // tokio::task::spawn(async move {
    //     if let Err(err) = conn.await {
    //         println!("Connection failed: {:?}", err);
    //     }
    // });

    // let authority = url.authority().unwrap().clone();

    // let path = url.path();
    // let req = Request::builder()
    //     .uri(path)
    //     .header(hyper::header::HOST, authority.as_str())
    //     .body(Empty::<Bytes>::new())?;

    // let mut res = sender.send_request(req).await?;

    // println!("Response: {}", res.status());
    // println!("Headers: {:#?}\n", res.headers());

    // // Stream the body, writing each chunk to stdout as we get it
    // // (instead of buffering and printing at the end).
    // while let Some(next) = res.frame().await {
    //     let frame = next?;
    //     if let Some(chunk) = frame.data_ref() {
    //         io::stdout().write_all(&chunk).await?;
    //     }
    // }

    println!("\n\nDone!");

    Ok(())
}

// async fn index_handler(req: HttpRequest, routesnew: Arc<Vec<Route>>) -> impl Responder {
//     // let routes = data.get_ref();
//     let path = req.match_info().query("tail");
//     println!("{}", path);
//     println!("req {}", req.uri());

//     for route in routesnew.iter() {
//         // Clone the host for the closure
//         let route_host = route.host.clone();
//         // Create an Arc for the host to ensure it lives long enough
//         let route_host_arc = Arc::new(route_host);
//         // Create a clone for the async block
//         let uri = format!("http://{}", route_host_arc);
//         let uri_clone = uri.clone();
//         // Spawn async block with a new Arc reference
//         let handle = tokio::spawn(async move {
//             let uri = uri_clone.parse::<Uri>().expect("Failed to parse URI");
//             if let Err(err) = fetch_url(uri).await {
//                 eprintln!("Failed to fetch URL {}: {}", route_host_arc, err);
//             }
//         });
//         // Ensure the handle doesn't drop until the end of the loop iteration
//         handle.await;
//     }

//     HttpResponse::Ok().body("Hey there!")
// }

#[get("/{tail:.*}")]
async fn hello(req: HttpRequest) -> String {
    let routes = read_user_from_file("routes.json").unwrap();
    let mounted = serde_json::to_string(&routes).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&mounted).unwrap();
    // let path = req.match_info().query("tail");
    // println!("{}", parsed["routes"][0]["host"]);
    let mounted_url = parsed["routes"][0]["host"].to_string();
    // .parse::<hyper::Uri>()
    // .unwrap();
    println!("req {}", mounted_url);
    // fetch_url(mounted_url).await;
    // HttpResponse::Ok().body(parsed["routes"][0]["host"])
    parsed["routes"][0]["host"].to_string()
}

#[get("/configuration")]
async fn configuration() -> String {
    let routes = read_user_from_file("routes.json").unwrap();
    let mounted = serde_json::to_string(&routes).unwrap();
    mounted
}

#[post("/{tail:.*}")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let routes = read_user_from_file("routes.json").unwrap();
    // let routesnew = Arc::new(read_user_from_file("routes.json").unwrap().routes);
    // println!("{:#?}", routesnew);
    // // Lê as rotas do arquivo JSON
    // let file = File::open("routes.json").expect("Failed to open routes.json");
    // let reader = BufReader::new(file);
    // // println!("{}",reader);
    // let routes: Vec<Route> = serde_json::from_reader(reader).expect("Failed to parse routes.json");
    // println!("{}", routes);
    // let mut path_tree = PathTree::new();

    // for route in &routes.routes {
    //     // let handler = match route.handler.as_str() {
    //     //     "index" => index,
    //     //     // "echo_query" => echo_query,
    //     //     _ => panic!("Unknown handler: {}", route.handler),
    //     // };
    //     path_tree.insert(&route.host.clone(), handler);
    // }
    // let cloned_routes = routes.routes.clone();
    HttpServer::new(move || {
        App::new()
            .service(echo)
            .service(configuration)
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
            .service(
                web::resource("/{tail:.*}")
                    //.get(|| HttpResponse::Ok()),
                    .route(web::get().to(|req: HttpRequest| {
                        let path = req.match_info().query("tail");
                        println!("{}", path);
                        println!("req {}", req.uri());
                        // println!("{:#?}", routesnew);
                        // index_handler(req, routesnew.clone());
                        let routesnew =
                            Arc::new(read_user_from_file("routes.json").unwrap().routes);
                        for route in routesnew.iter() {
                            println!("{:#?}", route);
                        }
                        //     // Clone the host for the closure
                        //     let route_host = route.host.clone();
                        //     // Create an Arc for the host to ensure it lives long enough
                        //     let route_host_arc = Arc::new(route_host);
                        //     // Create a clone for the async block
                        //     let uri = format!("http://{}", route_host_arc);
                        //     let uri_clone = uri.clone();
                        //     // Spawn async block with a new Arc reference
                        //     let handle = tokio::spawn(async move {
                        //         let uri = uri_clone.parse::<Uri>().expect("Failed to parse URI");
                        //         if let Err(err) = fetch_url(uri).await {
                        //             eprintln!("Failed to fetch URL {}: {}", route_host_arc, err);
                        //         }
                        //     });
                        //     // Ensure the handle doesn't drop until the end of the loop iteration
                        //     handle
                        // }
                        // for route in &routes.routes {
                        //     fetch_url(route).await;
                        // }
                        // Ok(format!("Welcome {}!", info.username))
                        HttpResponse::Ok()
                        // match path_tree.find(&path) {
                        //     Some(handler) => handler(req),
                        //     None => HttpResponse::NotFound().body("Not Found"),
                        // }
                    })),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
