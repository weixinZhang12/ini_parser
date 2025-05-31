
use crate::parser::Parser;

mod value;
mod parser;
mod types;

fn main() {
    let mut s = Parser::new("./file/server.ini").unwrap();
    let mut parser = s.parser().unwrap();
    let r = s.ini_table();
    println!("{:#?}", r);    
}
 