use std::io::BufReader;
use crate::glyph;

//glyph instance

pub fn new(lines: std::io::Lines<BufReader<std::fs::File>>) {
    let _ = glyph::new(lines); //context builder
    //instance.start();

    //instance
}