use std::{
    env, 
    fs, 
    io::{prelude::*, BufReader}, 
    net::{TcpListener, TcpStream}, 
    //path::{Path, PathBuf} 
};




fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&stream);
    
    // let raw_http_request: Vec<_> = buf_reader
    // .lines()
    // .map(|result|result.unwrap())
    // .take_while(|line| !line.is_empty())
    // .collect();
    // println!("Request: {raw_http_request:#?}");


    //let response_text = String::new();
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let response_text = create_response();
        stream.write_all(response_text.as_bytes()).unwrap();
    } 
    else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response_text = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        stream.write_all(response_text.as_bytes()).unwrap();
    }

    // response
    //println!("Response: {response_text:#?}");

}

fn create_response() -> String {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    return response;
}
