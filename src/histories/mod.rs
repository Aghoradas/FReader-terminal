use std::collections::BTreeMap;


#[derive(Default, Debug)]
pub struct ComHistory {
    history_collection: BTreeMap<i32, String>,
}

impl ComHistory {
    pub fn add_line(&mut self, line: &String) {
        let size32 = i32::try_from(self.history_collection.len()+1).unwrap();
        self.history_collection.insert(size32, line.to_string());
    }
    pub fn drop_line(&mut self) {
        let which = i32::try_from(self.history_collection.len()).unwrap();
        self.history_collection.remove(&which).unwrap();
    }
    pub fn show_history(&mut self) {
        for (key, value) in &self.history_collection {
            println!(
                "line: {} - {}",
                key,
                value
            );
        }
    }
    pub fn get_history(&self, num: usize) -> String {
        let mut entry = String::new();
        if num <= self.history_collection.len() {
            if self.history_collection.len() >= num {
                let num32 = &i32::try_from(num).unwrap();
                entry = self
                    .history_collection
                    .get(num32)
                    .expect("-no entry")
                    .to_string();
            }
        }
        entry
    }/*
    pub fn get_size(&self, num: usize) -> usize {
        let num32 = &i32::try_from(num - 1).unwrap();
        let size = self.history_collection[num32].trim().len();
        size
    }*/
    pub fn map_size(&self) -> usize {

        self.history_collection.len()
    }/*
    pub fn is_empty(&self) -> bool {
        let empty = self.history_collection.is_empty();
        empty
    }*/
}

pub fn new_hist() -> ComHistory {

    ComHistory {
        history_collection: Default::default(),
    }
}
