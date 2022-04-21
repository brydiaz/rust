use WebServer::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::io::BufReader;
use std::env;
use std::fs::File;
use std::io::Write;


fn main() {
    let args: Vec<String> = env::args().collect();
    let number_threads = args[1].parse::<usize>().unwrap();
    println!("{:?}",args);
    let mut ip = "127.0.0.1:".to_owned();
    ip.push_str(&args[3]);

    let listener = TcpListener::bind(ip).unwrap();
    let pool = ThreadPool::new(number_threads);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            let args: Vec<String> = env::args().collect();

            let resources_path = &args[2];

            handle_connection(stream, resources_path.to_string());
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, resources_path: String) {
  

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let get = b"GET / HTTP/1.1\r\n";
    let post = b"POST / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        let data = String::from_utf8(buffer.to_vec()).unwrap();
        let data = data.trim_matches(char::from(0));
        let mut split = data.split(" ");        
        let vec: Vec<&str> = split.collect();
        let mut name = "/".to_string();
        for c in vec[5].chars() {
            if c != '\r' && c != '\n' && c != '0' && c != '0' && c != '1'{
                name.push_str(&c.to_string());
            }
        }
        let final_name = resources_path + &name;
        println!("{:?}", final_name);
        ("HTTP/1.1 200 OK", final_name)
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", resources_path+"/hello.html")
    } else if buffer.starts_with(post) {
        let data = String::from_utf8(buffer.to_vec()).unwrap();
        let data = data.trim_matches(char::from(0));
        let resources = "resources";
        println!("{:?}","holi1");
        fs::write("resources/post.html", data).expect("Unable to write file");
        println!("{:?}","holi2");
        ("HTTP/1.1 200 OK", resources.to_owned()+"/post.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", resources_path+"/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}