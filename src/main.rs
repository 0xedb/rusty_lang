mod lexer;

// #[cfg(test)]
// mod tests {
//     use crate::lexer::Lexer;

//     #[test]
//     fn it_works() {
//         let l = Lexer::new(String::from("let a = 300;"));

//         println!("{:?}", l);
//     }
// }

use lexer::Lexer;

fn main() {
    let l = Lexer::new(String::from("let a = 300"));
    println!("{:?}", l);
}
