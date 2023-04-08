use std::fs;
use std::env;

fn main() {
   let args: Vec<String> = env::args().collect();

   let v = args.get(1).unwrap_or_else(|| String::from("failed").as_str);

   println!("{v}");

   if let Some(f) = args.get(1) {
       let contents = fs::read_to_string(f).expect("failed");

       println!("{contents}");
   } else {
       println!("file path arg missing");
   }
}
