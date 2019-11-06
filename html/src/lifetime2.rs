// lifetime1.rs
// Lifetime

fn main() {
   let mut s:String = String::from("hey");
   print!("{}\n", s);
   s = "hello".to_string();
   let y = &mut s;
   y.push_str(" hello2");
   print!("there\n");
   let mut s:i32 = 12;
   let y = &mut s;
   print!("{}", y);
   
   let a = &mut s;
}