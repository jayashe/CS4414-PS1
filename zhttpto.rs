//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut visitor_count:   int = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    

    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];  

            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));
            //get first line of request
            //array.split based on newlines
            let request_lines: ~[&str] = request_str.split('\n').collect(); 
            //array.split based on spaces
            let request_parts: ~[&str] = request_lines[0].split(' ').collect(); 
            //See if response is of type GET
            if (request_parts[0] == "GET") {
                let path_str = "." + request_parts[1];
                //update visitor count
                unsafe {
                    visitor_count += 1;     
                }
                //if no file is selected serve the default people counter
                if (path_str == ~"./") {
                    unsafe {
                        let response: ~str = 
                            ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                             <doctype !html><html><head><title>Hello, Rust!</title>
                             <style>body { background-color: #111; color: #FFEEAA }
                                    h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                                    h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                             </style></head>
                             <body>
                             <h1>Greetings, Krusty!</h1>
                             <h2>Visitors: " + visitor_count.to_str() + "</h2>
                             </body></html>\r\n";
                        stream.write(response.as_bytes()); 
                    }
                //Checks file type. Only serves HTML files.
                } else if (path_str.slice_from(path_str.len() - 4) != "html") {
                    let forbidden_string: ~str = 
                    ~"HTTP/1.1 403 Forbidden\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                     <doctype !html><html><head><title>Hello, Rust!</title>
                     <body>
                     <h1>403 Forbidden.</h1>
                     </body></html>\r\n";                    
                    stream.write(forbidden_string.as_bytes());
                    println!("Not an HTML File. 403 Forbidden!");
                //if the file exists, serve an HTML file
                } else {
                    //see if path exists in CWD (curr working dir) (match f)
                    let file_path= Path::new(path_str.clone());
                    //This error handling section from the PS Comments thanks to 'Erik'
                    io_error::cond.trap(|_| {
                        println("We hit an error...");
                        // hoo-boy...
                    }).inside(|| {
                        match File::open_mode(&file_path, Open, Read) {
                            Some(mut file) => {
                                let message_bytes: ~[u8] = file.read_to_end();
                                stream.write(message_bytes);
                            }
                            None => {
                                println("File does not exist. 404!."); 
                                let no_such_file_string: ~str = 
                                ~"HTTP/1.1 404 Not Found\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                                 <doctype !html><html><head><title>Hello, Rust!</title>
                                 <body>
                                 <h1>404 File not Found.</h1>
                                 </body></html>\r\n";
                                stream.write(no_such_file_string.as_bytes());
                            }
                        };
                    });

                }
            println!("Connection terminates.");
            }
           
        }
    }
}
