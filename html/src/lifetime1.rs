// lifetime1.rs
// Lifetime

fn main() {
   let mut i:i32 = 1;
   let mut ref_i = &mut i;
   let second_tier_ref = &mut ref_i;
   print!("{}", **second_tier_ref)
}