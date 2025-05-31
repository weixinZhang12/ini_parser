use std::{collections::HashMap, error::Error, fs, io, path::Path};

use crate::{
    types::{IniTable, Selection},
    value::Value,
};
// #[derive(Debug)]
pub struct Parser {
    content: String,
    ini_table: IniTable,
}

impl Parser {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let s = fs::read_to_string(path)?;
        let temp = Self {
            content: s,
            ini_table: HashMap::new(),
        };
        Ok(temp)
    }
    pub fn parser(&mut self) -> Result<(), Box<dyn Error>> {
        let mut selection_map: Selection = Selection::new();
        let mut last_seciton = String::new();
        let mut selection = String::new();

        for line in self.content.lines() {
            let line = line;
            let mut key = String::new();
            let mut value = String::new();
            // 查看该行是否有selection有那么使用填写的selection，否则使用空字

            let err = Box::new(io::Error::new(io::ErrorKind::InvalidData, "无效的数据"));
            if let Some(v) = Self::get_selection(line) {
                if last_seciton.is_empty() {
                    last_seciton = v.clone();
                } else {
                    last_seciton = selection.clone();
                }
                selection = v;
            }
            if let Some(v) = Self::get_key(line) {
                key = v
            }
            if let Some(v) = Self::get_value(line) {
                value = v
            }
            if selection != last_seciton {
                self.ini_table.insert(last_seciton.clone(), selection_map);
                selection_map = HashMap::new();
                last_seciton=selection.clone();
            } else if !key.is_empty() {
                selection_map.insert(key.trim().to_string(), Value::from(&value));
            }
        }
        self.ini_table.insert(selection, selection_map);
        Ok(())
    }

    fn get_selection(line: &str) -> Option<String> {
        let mut selection = String::new();
        let mut end_idnex = 0;
        if line.starts_with('[') {
            match line.find("]") {
                Some(v) => end_idnex = v,
                None => {
                    return None;
                }
            }
            selection = line[1..end_idnex].to_string();
        } else {
            return None;
        }
        Some(selection)
    }

    fn get_key(line: &str) -> Option<String> {
        let eq_idnex_result = line.find("=");
        let mut key = String::new();
        let mut eq_index = 0;
        match eq_idnex_result {
            Some(v) => eq_index = v,
            None => return None,
        }
        key = line[0..eq_index].to_string();
        Some(key)
    }
    fn get_value(line: &str) -> Option<String> {
        let eq_idnex_result = line.find("=");
        let mut value = String::new();
        let mut eq_index = 0;
        match eq_idnex_result {
            Some(v) => eq_index = v,
            None => return None,
        }
        value = line[eq_index + 1..].to_string();
        Some(value)
    }
    
    pub fn ini_table(&self) -> &IniTable {
        &self.ini_table
    }
}
