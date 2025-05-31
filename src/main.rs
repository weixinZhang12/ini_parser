use value::Value;

use crate::parser::Parser;

mod value;
mod parser;
mod types;

fn main() {
    let s=Parser::new("./file/server.ini").unwrap();
    let r=s.parser().unwrap();
    println!("{:?}",r);
}
 