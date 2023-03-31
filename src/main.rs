// TODO:
//  * Build process
//      - Go over all html files and add compenents in raw html. Grab from partials folder.
//      - Make create and add to output directory

use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use Rustic::build_files;
use Rustic::generate_paths;

fn main() {
    build_files::build();

    // TODO change this to get the local ip address on build
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let paths: HashMap<String, String> = generate_paths::generate_static_paths();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("./paths/index.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let slug: String = extract_slug(request_line);

        let file_name = paths.get(&slug);

        match file_name {
            Some(_) => {
                let status_line = "HTTP/1.1 200 OK";
                let contents = fs::read_to_string(file_name.unwrap()).unwrap();
                let length = contents.len();

                let response =
                    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

                stream.write_all(response.as_bytes()).unwrap();
            }
            None => {
                let status_line = "HTTP/1.1 200 OK";
                let contents = fs::read_to_string("./paths/404.html").unwrap();
                let length = contents.len();

                let response =
                    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    }
}

fn extract_slug(rl: String) -> String {
    if rl.contains(".html") {
        for character in rl[5..].split(".") {
            if !character.is_empty() {
                return character.trim().to_owned();
            }
        }
    } else {
        for character in rl[5..].split("H") {
            if !character.is_empty() {
                return character.trim().to_owned();
            }
        }
    }

    return "".to_owned();
}
