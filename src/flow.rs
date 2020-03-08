#![allow(dead_code)]
#![allow(unused_variables)]
#![warn(unused_imports)]

// use std::mem;


pub fn if_statements() {

    println!("\nFrom flow.rs module");
    let temp = 21;

    if temp > 30 {
        println!("Really hot outside");
    } else if temp < 10 {
        println!("Really cold outside");
    } else {
        println!("It's between 10 & 30, temp is  {}", temp)
    }

    let day = if temp > 20 {"Sunny"} else {"Cloudy"};
    println!("Today is {}", day);

    println!("...it is {}",
      if temp > 20 {"Hot"} else if temp < 10 {"Cold"} else {"Ok"});
}

pub fn while_and_loop() {

   let mut x = 1;

   while x < 1000 {
        x *=2 ;
        if x == 64 { continue; }
        println!("x = {}", x)
   }
   let mut y = 1;
   loop {  // while true
        y *= 2;
        println!("y = {}", y);
        if y == 1<<10 { break; }
   }
}
