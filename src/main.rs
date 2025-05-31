use value::Value;

mod value;
mod parser;
mod types;

fn main() {
    let s=Value::Double(1.23).to_string();
    println!("{}",s)
}
 