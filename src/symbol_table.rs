use std::collections::HashMap;

pub struct SymbolTable {
    pub table: HashMap<String, usize>,
    pub alloc_pos: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut t = Self {
            table: HashMap::new(),
            alloc_pos: 16,
        };
        for i in 0..16 {
            t.add_entry(&format!("R{i}"), i);
        }
        t.add_entry("SP", 0);
        t.add_entry("LCL", 1);
        t.add_entry("ARG", 2);
        t.add_entry("THIS", 3);
        t.add_entry("THAT", 4);
        t.add_entry("SCREEN", 16384);
        t.add_entry("KBD", 24576);
        t
    }

    pub fn add_entry(&mut self, k: &str, v: usize) {
        self.table.insert(k.to_string(), v);
    }

    pub fn contains(&self, k: &str) -> bool {
        self.table.contains_key(k)
    }

    pub fn get_address(&self, k: &str) -> usize {
        self.table.get(k).unwrap().clone()
    }
}
