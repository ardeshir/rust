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
        if x == 1024 { continue; }  // don't print 1024
        println!("x = {}", x);
   }

   let mut y = 1;
   loop {  // while true
        y *= 2;
        println!("y = {}", y);
        if y == 1<<10 { break; }
   }
}

pub fn for_loop() {
    
    for x in 1 ..13 {
        if x == 11 { continue; }
        if x == 12 { break; }
        println!("x is {}", x)
    }

    for (x,y) in (1..13).enumerate() {
        if x == 11 { continue; }
        if y == 11 { continue; }
        if x == 12 { break; }
        if y == 12 { break; }
        println!("x is {} & y is {}", x, y)
    }


   for (pos, y) in (30..41).enumerate() {
       println!("index: {}, y: {}", pos, y);
   }
}
