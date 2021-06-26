use bark_frontend::frontend;
use bark_core::io;

fn main() {
    println!("Loading today...");
    let file = io::load().unwrap();

    println!("Launching frontend...");
    let fe = frontend::new(file);
}
