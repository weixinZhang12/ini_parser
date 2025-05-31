use std::error::Error;
#[derive(Debug)]
pub enum Value {
    Bool(bool),
    String(String),
    Int(i32),
    Double(f64),
}

impl Value {
    pub fn from(s: &str) -> Self {
        match Self::from_bool(s) {
            Ok(v) => return v,
            Err(_) => {}
        }
        if s.contains('.') {
            match Self::from_double(s) {
                Ok(v) => return v,
                Err(_) => {}
            }
        } else {
            match Self::from_int(s) {
                Ok(v) => return v,
                Err(_) => {}
            }
        }

        Value::String(s.to_string())
    }

    pub fn from_double(s: &str) -> Result<Value, Box<dyn Error>> {
        let s = s.trim();
        let t = s.parse::<f64>()?;
        let t = Value::Double(t);
        Ok(t)
    }
    pub fn from_bool(s: &str) -> Result<Value, Box<dyn Error>> {
        let s = s.trim();

        let t = s.parse::<bool>()?;
        let t = Value::Bool(t);
        Ok(t)
    }
    pub fn from_int(s: &str) -> Result<Value, Box<dyn Error>> {
        let s = s.trim();

        let t = s.parse::<i32>()?;
        let t = Value::Int(t);
        Ok(t)
    }
    pub fn to_string(&self) -> String {
        match self {
            Value::Bool(v) => v.to_string(),
            Value::String(v) => v.clone(),
            Value::Int(v) => v.to_string(),
            Value::Double(v) => v.to_string(),
        }
    }
}
