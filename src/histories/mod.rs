
use std::collections::HashMap;

pub struct ComHistory {
    history_collection: HashMap<i32, String>,
}

impl ComHistory {
    pub fn add_line(&mut self, line: &String) {
        let size32 = i32::try_from(self.history_collection.len())
            .unwrap();
        self.history_collection.insert(size32, line.to_string());
    }
    pub fn show_history(&self) {
        for num in 0..self.history_collection.len() {
            let num32 = &i32::try_from(num).unwrap();
            println!("line: {} - {}", num+1, self.history_collection
                .get(num32)
                .unwrap()
                .trim());
        }
    }
    pub fn get_history(&self, num: usize) -> String {
        let mut entry = String::new();
        if self.history_collection.len() >= num {
            let num32 = &i32::try_from(num-1).unwrap();
            entry = self.history_collection.get(num32)
                .expect("-no entry")
                .to_string();
        }
        entry
    }
    pub fn get_size(&self, num: usize) -> usize {
        let num32 = &i32::try_from(num-1).unwrap();
        let size = self.history_collection[num32].trim().len();
        size
    }
    pub fn map_size(&self) -> usize {
        let size = self.history_collection.len();
        size
    }
    pub fn is_empty(&self) -> bool {
        let empty = self.history_collection.is_empty();
        empty
    }
}

pub fn new_hist() -> ComHistory {
    let command_hist: HashMap<i32, String> = HashMap::new();

    ComHistory {
        history_collection: command_hist,
    }
}
