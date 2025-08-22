fn main() {
    
    // + plus sign
    // - minus sign
    
    // i8, i16, i32, i64, i128, and isize.
    // u8, u16, u32, u64, u128, and usize.
    
    // bits = 8 bit =  1 byte
    
    // Computer Architecture에 따른 것
    
    
    let my_number: u8 = 100; // 255
    let my_other_number = 50;
    let third_number = my_number + my_other_number;
    
    // type inference

  /*
   Compiling playground v0.0.1 (/playground)
warning: unused variable: `third_number`
  --> src/main.rs:16:9
   |
16 |     let third_number = my_number + my_other_number;
   |         ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_third_number`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `playground` (bin "playground") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.82s
     Running `target/debug/playground`
  */
}
