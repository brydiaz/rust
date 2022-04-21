use hyper::{Client,Body, Method, Request, Uri};
pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub type GenericResult<T> = std::result::Result<T, GenericError>;
use hyper::body;
use std::env;
use std::fs;


async fn get(url:&str, filename: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let request = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("accept", "application/json")
        .body(Body::from(filename.to_string())).unwrap();
    let client = Client::new();
    let resp = client.request(request).await.unwrap();
    println!("Response GET: {}", resp.status());
    let bytes = body::to_bytes(resp.into_body()).await.unwrap();
    //println!("GOT BYTES: {}", std::str::from_utf8(&bytes).unwrap() );
    Ok(())
}

async fn post(url: &str, message: &str) ->  Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let data = fs::read_to_string(message).expect("Unable to read file");


    let request = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("accept", "application/json")
        .header("Content-type", "application/json; charset=UTF-8")
        .body(Body::from(data)).unwrap();
    let client = Client::new();
    let resp = client.request(request).await.unwrap();
    let bytes = body::to_bytes(resp.into_body()).await.unwrap();
    
    println!("GOT BYTES: {}", std::str::from_utf8(&bytes).unwrap() );
    Ok(())

}

#[tokio::main]
async fn main() ->  Result<(), Box<dyn std::error::Error + Send + Sync>> { 
    let args: Vec<String> = env::args().collect();
    let choice = &args[2];
    let host = &args[1];
    let arguments;
    if args.len() > 3 {
        arguments = &args[3];
    } else {
        arguments = &args[0];
    }
    let post_method = "POST";
    let get_method = "GET";

    if choice == post_method {
        post(host, arguments).await;
    } else if choice == get_method {
        get(host,arguments).await;
    }

    Ok(())
}