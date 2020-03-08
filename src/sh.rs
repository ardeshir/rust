#![allow(dead_code)]
#![allow(unused_variables)]
#![warn(unused_imports)]
use std::mem;

struct  Point {
 // size 16 bytes
 x: f64,
 y: f64

}

fn origin() -> Point {

  Point{x: 0.0, y: 0.0}

}
pub fn stack_and_heap() {

    println!("\nFrom inside sh.rs pub fn stack_and_heap()");

    let p1 = origin();
    let p2 = Box::new( origin() ); // size 8 bytes
    // p2 is a pointer on heap

    println!("p1 takes up {} bytes on the heap: p1.x = {}", mem::size_of_val( &p1 ), p1.x);
    println!("p2 takes up {}  bytes on the stack: p2.x = {}", mem::size_of_val( &p2 ), p2.x);

    let p3 = *p2;
    println!("{}", p3.x)

}
