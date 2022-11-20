use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::types::PyTuple;
use std::borrow::Cow;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn request_to_py(test: &Cow<'_, str>) -> PyResult<()> {
    let req_str = &test.to_string();
    
    // println!("this is test to string ");
    // print_type_of(&a);
    // println!("{}", &a);

    let req_split_array = req_str.split(":");

    for word in req_split_array {
        println!("{}", word)
    }

    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code(
            py,
            "def example(*args, **kwargs):
                if args != ():
                    print('called with args', args)
                if kwargs != {}:
                    print('called with kwargs', kwargs)
                if args == () and kwargs == {}:
                    print('called with no arguments')",
            "",
            "",
        )?.getattr("example")?.into();

        // call object without any arguments
        // fun.call0(py)?;

        // call object with PyTuple
        // let args = PyTuple::new(py, &[arg1, arg2, arg3]);
        // fun.call1(py, args)?;

        // pass arguments as rust tuple
        Ok(())
    })
}

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            println!("asdfasdfasdfasdfasdf");
            let req_str = String::from_utf8_lossy(&buf);
            request_to_py(&req_str);
            // println!("{}", req_str);
        },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    print_type_of(response);
    
    match stream.write(response) { 
        Ok(_) => {
            println!("Response sent");
        },
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    println!("handle read");
    handle_read(&stream);
    println!("handle write");
    handle_write(stream);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening for connections on port {}", 8080);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
