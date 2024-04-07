use actix_web::Result;

pub async fn fetch_get_url(url: &str) -> Result<std::string::String, actix_web::Error> {
    let client = reqwest::Client::new();
    let mut response: String = "".to_string();
    let mut error = false;
    let mut retrys = 5;
    do_loop!({
    response = match client.get(url).send().await {
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
