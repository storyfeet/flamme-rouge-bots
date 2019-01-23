mod track;
use crate::track::{Track,Hill,Rider};




fn main() {
    println!("Hello, world!");

    use self::Hill::*;
    let mut tk = Track::from_rd(vec![(Flat, 5), (Up, 3), (Flat, 20), (Down, 2), (Flat, 9)]);

    tk.add_riders(3);
    tk.move_riders(Vec::new());
    tk.print();
    
}
