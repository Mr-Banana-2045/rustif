use rand::thread_rng;
use rand::seq::SliceRandom;
use std::env;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = if !args[1..].is_empty() {
        format!("-$ {}", args[1..].join(" "))
    } else {
        String::from("-$ ")
    };
    let text_length = result.len();
    let box_length = text_length + 2;
    let num = 4;
    let items = [
                 r#"  ________
  \      /
   \+  */
    \--/
     \/"#.blue(),
                 r#" /\____/\
 |      |
 | #  # |
 |  --  |
  \____/"#.green(),
                 r#" _________
 |       |
 | X   X |
 \  > <  /
  | ### |
  \_____/"#.yellow(),
                 r#"   |||||||||
   |       |
 |=|       |=|
   |       |
   |  - -  |
   |   _   |
    \_____/"#.red()];

    let mut rng = thread_rng();
    let random_item = items.choose(&mut rng).unwrap();
    println!(" {} ", "_".repeat(box_length));
    println!("| {} |", result.green());
    println!(" {} ", "-".repeat(box_length));
    for i in 0..num {
        println!(" {:width$}\\","", width = i);
    }
    println!("{}", random_item);
}
