// use rfnd-hello-world::HELLO_WORLD;

// // ts: export const HELLO_WORLD: string = 'Hello world!';
// pub const HELLO_WORLD: &str = "Hello world!";

// fn main() {
//     println!("Required: {}.", HELLO_WORLD);
// }
// // fn main() {
// //   println!("Hello, world!");
// // }

// read file
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::from_utf8;

fn main() {
    let mut file = match File::open("hello.txt") {
        Err(err) => panic!("Couldn't open: {}", err.description()),
        Ok(value) => value,
    };

    let stat = match file.metadata() {
        Err(err) => panic!("Couldn't get stat: {}", err.description()),
        Ok(value) => value,
    };

    let mut buffer = vec![0; stat.len() as usize];

    match file.read(&mut buffer) {
        Err(err) => panic!("Couldn't read: {}", err.description()),
        Ok(_) => (),
    };

    let data = match from_utf8(&buffer) {
        Err(err) => panic!("Couldn't convert buffer to string: {}", err.description()),
        Ok(value) => value,
    };

    println!("Content is: {}", data);
}
