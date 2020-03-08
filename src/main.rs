use std::mem;

fn main() {
    println!("Building binary data app:");
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
    let e = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));
}
