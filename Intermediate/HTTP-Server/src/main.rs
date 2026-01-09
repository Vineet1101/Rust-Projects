#[allow(unused_imports)]
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
// use codecrafters_http_server::ThreadPool;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    // let pool=ThreadPool::new(4);

    for stream in listener.incoming() {
        
        std::thread::spawn(||handle_connection(stream.unwrap()));
        
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("{:#?}", http_request);
    
    let request_line: Vec<_> = http_request[0].split('/').collect();
    println!("{:?}", request_line);

    let user_agent=request_line[1].split(' ').collect::<Vec<_>>()[0];
    println!("{user_agent}");
    
    if request_line[1] == " HTTP" {

        let success_response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(success_response.as_bytes()).unwrap();

    } else if request_line[1] == "echo" {
        
        let request_uri = request_line[2].split(' ').collect::<Vec<_>>();
        println!("{:?}", request_uri);
        
        let success_status_line = "HTTP/1.1 200 OK";
        let contents = request_uri[0];
        let length = contents.len();
        let success_response_with_content = format!(
            "{success_status_line}\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        
        stream
            .write_all(success_response_with_content.as_bytes())
            .unwrap();
    
    }else if user_agent=="user-agent"{
            println!("This is user agent block");
        let headers = &http_request[http_request.len()-1];
        println!("{:?}", headers);
        
        let success_status_line = "HTTP/1.1 200 OK";
        let contents = headers.split(' ').collect::<Vec<_>>()[1];
        println!("{contents}");
        
        let length = contents.len();

        let success_response_with_content = format!(
            "{success_status_line}\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        
        stream
            .write_all(success_response_with_content.as_bytes())
            .unwrap();
    } else {

        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
        
    }
}
