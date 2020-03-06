//Library for creating simple markov-chain text simulators

mod textsim {
    use std::collections::HashMap;
    use std::io::{BufReader,BufRead};
    use std::fs;

    pub struct Model{
        name: String,
        chain: HashMap<String, Vec<String>>,
        order: u32,
    }

    impl Model{
        pub fn new(nm: String, ord: u32) -> Self {
            Model {
                name: nm,
                chain: HashMap::new(),
                order: ord
            }
        }
        //self note, consider making a corpus set and have entries be references to words in corpus set
        pub fn process_text(&mut self, path: &str) {
            let mut text_vec: Vec<String> = self.file_to_string_vec(path);
            for i in 0..text_vec.len() {
                if !self.chain.contains_key(&text_vec[i]){
                    self.chain.insert(text_vec[i].clone(), Vec::<String>::new());
                }
                for j in 0..self.order {
                    if i + j < text_vec.len() {

                    }
                }
                if i > 0 {
                    if let Some(prev) = self.chain.get_mut(&text_vec[i-1]){
                        prev.push(text_vec[i].clone());
                    }
                }
            }
        }

        pub fn file_to_string_vec(&self, path: &str) -> Vec<String> {
            let text = fs::read_to_string(path).expect("File Read Error");
            let mut string_vec = Vec::new();
            string_vec = text.split_whitespace()
                             .map(|s| s.to_string())
                             .collect();
            string_vec
        }
    }

    struct Word {
        word: String,
        count: u32
    }

    struct Fields {
        
    }
}
