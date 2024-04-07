use actix_web::Result;
use hyper;
use std::error::Error;
macro_rules! do_loop {(
    $body:block while $cond:expr
) => ({
    let mut first = true;
    while ::core::mem::replace(&mut first, false) || $cond
        $body
})}

// #[tokio::main]
// async fn get2() -> Result<(), Box<dyn Error>> {
//     // Create a new client object
//     let mut client = hyper::Client::new();

//     let req = hyper::Request::builder()
//         .method(hyper::Method::GET)
//         .uri("http://httpbin.org/ip")
//         .header("user-agent", "the-awesome-agent/007")
//         .body(hyper::Body::from(""))?;

//     // Pass our request builder object to our client.
//     let resp = client.request(req).await?;

//     // Get the response body bytes.
//     let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;

//     // Convert the body bytes to utf-8
//     let body = String::from_utf8(body_bytes.to_vec()).unwrap();

//     println!("{}", body);

//     Ok(())
// }

pub(crate) async fn get(url: &str) -> Result<std::string::String, actix_web::Error> {
    let client = reqwest::Client::new();
    let mut response: String = "".to_string();
    let mut error = false;
    let mut retrys = 5;
    do_loop!({
    response = match client.get(url)
    //.(std::time::Duration::from_millis(2000))
    .timeout(std::time::Duration::from_millis(4000))
    .send().await {
        Ok(resp) => {
            error = false;
            resp.text().await.unwrap()
        },
        Err(err) => {
            error = true;
            println!("Error: {}", err);
            "".to_string()
        }
    };
    if retrys == 0 {
        retrys = 0;
        continue;
    }
    retrys -= 1;
    println!("{}", retrys);
    } while retrys == 0 && error == true);
    println!("{}", response);
    // if (error == true) {
    //     return panic!("Error: {}", err);
    // }

    Ok(response)
}
pub(crate) async fn put(url: &str) -> Result<std::string::String, actix_web::Error> {
    let client = reqwest::Client::new();
    let mut response: String = "".to_string();
    let mut error = false;
    let mut retrys = 5;
    do_loop!({
    response = match client.put(url).send().await {
        Ok(resp) => {
            error = false;
            resp.text().await.unwrap()
        },
        Err(err) => {
            error = true;
            println!("Error: {}", err);
            "".to_string()
        }
    };
    if retrys == 0 {
        retrys = 0;
        continue;
    }
    retrys -= 1;
    println!("{}", retrys);
    } while retrys == 0 && error == true);
    println!("{}", response);
    // if (error == true) {
    //     return panic!("Error: {}", err);
    // }

    Ok(response)
}
pub(crate) async fn post(url: &str) -> Result<std::string::String, actix_web::Error> {
    let client = reqwest::Client::new();
    let mut response: String = "".to_string();
    let mut error = false;
    let mut retrys = 5;
    do_loop!({
    response = match client.put(url).send().await {
        Ok(resp) => {
            error = false;
            resp.text().await.unwrap()
        },
        Err(err) => {
            error = true;
            println!("Error: {}", err);
            "".to_string()
        }
    };
    if retrys == 0 {
        retrys = 0;
        continue;
    }
    retrys -= 1;
    println!("{}", retrys);
    } while retrys == 0 && error == true);
    println!("{}", response);
    // if (error == true) {
    //     return panic!("Error: {}", err);
    // }

    Ok(response)
}
pub(crate) async fn delete(url: &str) -> Result<std::string::String, actix_web::Error> {
    let client = reqwest::Client::new();
    let mut response: String = "".to_string();
    let mut error = false;
    let mut retrys = 5;
    do_loop!({
    response = match client.put(url).send().await {
        Ok(resp) => {
            error = false;
            resp.text().await.unwrap()
        },
        Err(err) => {
            error = true;
            println!("Error: {}", err);
            "".to_string()
        }
    };
    if retrys == 0 {
        retrys = 0;
        continue;
    }
    retrys -= 1;
    println!("{}", retrys);
    } while retrys == 0 && error == true);
    println!("{}", response);
    // if (error == true) {
    //     return panic!("Error: {}", err);
    // }

    Ok(response)
}
