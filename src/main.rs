#![allow(dead_code)]
#![allow(unused_variables)]
#![warn(unused_imports)]

mod sh;
mod flow;
use std::mem;

fn bitops() {

 println!("\nDoing bitwise operations\n");

 let c = 1 | 2 ; // | OR & AND ^ XOR ! NOR
                 // 01 OR 10 = 11 == 3_10
 println!("c: 1 | 2 = {} , size of c = {} bytes", c, mem::size_of_val(&c));

 let two_to_10 = 1 << 10;  // >>
 println!("2^10 = {}", two_to_10);

 // logical
 let pi_less_4 = std::f64::consts::PI < 4.0; // true
 // > <= >=
 println!("Pi < 4 = {}", pi_less_4);
 let x = 5;
 let x_is_5 = x == 5; // true

 println!(" x = {} x == 5: {} ", x, x_is_5);

}

fn data_types() {
    println!("\nBuilding binary data app:");
    let mut c = 123456789; // 32-bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after mutation", c);
    // i8 u8 i16 u16 i32 u32 i64 u64
    let z:isize = 123;  // isize/usize
    let size = mem::size_of_val(&z);
    println!("z = {}, size = {}, {}-bit OS", z, size, size * 8);

    let d:char = 'd';
    // let x = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));
    let e:f32 = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let f = false;
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));

}

fn ops() {

  // arithmetic
  println!("\nDoing Arithmetic: \n");
  let mut a = 2+3*4;
  println!("a = {}, size = {} bytes", a, mem::size_of_val(&a));
  a = a+1;  // rust doesn't support ++ or --
  a -= 2; // a = a - 2
          // -= += *= /=   %=
 println!("a = {}, size = {} bytes", a, mem::size_of_val(&a));
 println!("Remainder of {} / {} = {}", a, 3, (a%3));
 let a_cubed = i32::pow(a,3);
 println!("{} cubed = {}, size = {} bytes", a, a_cubed, mem::size_of_val(&a));

 let b = 2.5;
 let b_cubed = f64::powi(b, 3);
 let b_to_pi = f64::powf(b, std::f64::consts::PI);
 println!("b = {}, cubed = {} {}^pi = {}", b, b_cubed, b, b_to_pi);

}

fn main() {

  //data_types();
  //ops();
  //bitops();
  //sh::stack_and_heap();
  // flow::if_statements();
  // flow::while_and_loop();
  flow::for_loop();

}
