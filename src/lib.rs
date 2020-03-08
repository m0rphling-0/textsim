//Library for creating simple markov-chain text simulators
#![allow(dead_code)]
pub mod textsim {
    extern crate rand;
    use rand::Rng;
    use std::path::Path;
    use std::collections::HashMap;
    //use std::io::{BufReader,BufRead};
    use std::fs;
    
    #[derive(Debug)]
    pub struct Chain {
        field: Vec::<HashMap<String, usize>>
    }
    
    impl Chain {
        pub fn new(order: usize) -> Chain {
            Chain {
                field: vec![<HashMap<String, usize>>::new(); order]
            }
        }
    }    

    #[derive(Debug)]
    pub struct ChainMap {
        c_map: HashMap::<String, Chain>
    }
    
    impl ChainMap {
        pub fn new() -> ChainMap {
            ChainMap {
                c_map: HashMap::<String, Chain>::new()
            }
        }
        
        pub fn chain_map_insert(&mut self, key: &str, order: usize) -> bool {
            if !self.c_map.contains_key(key){
                self.c_map.insert(key.to_string(), Chain::new(order));
                true
            }
            else {
                false
            }
        }

        pub fn sub_chain_insert(&mut self, key: &str, sub: &str, ord: usize) {
            self.c_map.get_mut(key)
                .unwrap()
                .field[ord]
                .entry(sub.clone().to_owned())
                .and_modify(|value| {*value += 1})
                .or_insert(1);
        }

        pub fn get_sub_chain(&self, key: &str, ord: usize) -> &HashMap<String, usize> {
            &self.c_map.get(key).unwrap().field[ord]
        }
    }

    #[derive(Debug)]
    pub struct Model{
        name: String,
        pub chain_map: ChainMap,
        order: usize,
        start_words: Vec<String>
    }

    impl Model{
        pub fn new(nm: String, ord: usize) -> Self {
            Model {
                name: nm,
                chain_map: ChainMap::new(),
                order: ord,
                start_words: Vec::<String>::new()
            }
        }
    
        pub fn process_text(&mut self, path: &Path) {
            let text_vec: Vec<String> = self.file_to_string_vec(path);
            for i in 0..text_vec.len() {
                let cur_word = &text_vec[i];
                if self.chain_map.chain_map_insert(cur_word, self.order){
                    let ch = match cur_word.chars().next() {
                        Some(c) if c.is_uppercase() => true,
                        _ => false
                    };
                    if ch {
                        self.start_words.push(cur_word.to_string());
                    }
                }
                for j in 0..self.order {
                    if i + j + 1 < text_vec.len() {
                        self.chain_map
                            .sub_chain_insert(cur_word, &text_vec[i+j+1], j);
                    }
                }
            }
        }

        pub fn file_to_string_vec(&self, path: &Path) -> Vec<String> {
            let text = fs::read_to_string(path).expect("File Read Error");
            let mut string_vec = Vec::new();
            string_vec = text.split_whitespace()
                             .map(|s| s.to_string())
                             .collect();
            string_vec
        }

        pub fn generate_text(&self, length: usize) -> Vec<String> {
            let mut join_map = HashMap::<String, usize>::new();
            let mut gen_text = Vec::<String>::new();
            gen_text.push(self.get_random_key());
            for i in 0..length {
                for j in 0..self.order{
                    if j <= i {
                        let cur_chain = self.chain_map.get_sub_chain(&gen_text[i - j], j);
                        for (key, val) in cur_chain {
                            join_map.entry(key.to_owned())
                                    .and_modify(|value| {*value += val})
                                    .or_insert(*val);
                        }
                    }
                }
                let word_nums: Vec<(String, usize)> = join_map.drain().collect();
                let mut probability_vec = Vec::<usize>::new();
                let mut prev: usize = 0;
                for word_num in &word_nums {
                    prev += word_num.1;
                    probability_vec.push(prev);
                }
                let mut rng = rand::thread_rng();
                let rand_num: usize = rng.gen_range(0, prev);
                for i in 0..probability_vec.len() {
                    if rand_num <= probability_vec[i] {
                        gen_text.push(word_nums[i].0.to_owned());
                        break;
                    }
                }
            }
            gen_text
        }

        fn get_random_key(&self) -> String {
            let mut rng = rand::thread_rng();
            let num: usize = rng.gen_range(0, self.start_words.len());
            self.start_words[num].clone()
        }
    }

}
