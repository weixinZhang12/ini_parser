use std::collections::HashMap;

use crate::value::Value;

pub type Selection=HashMap<String,Value>;
pub type IniTable=HashMap<String,Selection>;