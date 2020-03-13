//Library for creating simple n-order markov-chain text simulators

pub mod textsim {
    extern crate rand;
    extern crate serde;
    extern crate serde_json;
    use serde::{Serialize, Deserialize};
    use rand::Rng;
    use std::path::Path;
    use std::collections::HashMap;
    use std::fs;
    
    #[derive(Serialize, Deserialize, Debug)]
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

    #[derive(Serialize, Deserialize, Debug)]
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
            &self.c_map.get(key)
                       .unwrap()
                       .field[ord]
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
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
    
        pub fn process_text(&mut self, path: &Path) -> Result<(), String>{
            match self.file_to_string_vec(path){
                Ok(text_vec) => {
                    self.proc_text(&text_vec);
                    Ok(())
                },
                Err(e) => return Err(e)
            }
        }

        fn proc_text(&mut self, text_vec: &Vec<String>) {
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

        pub fn file_to_string_vec(&self, path: &Path) -> Result<Vec<String>, String> {
            if let Ok(text) = fs::read_to_string(path){
                //create set of chars to strip from strings
                let strip_set: &[_] = &['(', ')', '"', '`'];
                //let mut split_string = Vec::new();
                //let mut string_vec   = Vec::new();
                let mut string_vec = text.split_whitespace()
                                  .map(|s| s.trim_matches(strip_set))
                                  .map(|s| s.to_string())
                                  .collect();
                /*for word in string_vec {
                    if 
                }*/
                Ok(string_vec)
            }
            else{
                Err("File Read Error".to_owned())
            }
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
                //contsruct a roulette wheel distribution to select a word with probability 
                //proportionate to its number of occurences
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
            gen_text.push(".".to_string());
            gen_text
        }

        fn get_random_key(&self) -> String {
            let mut rng = rand::thread_rng();
            let num: usize = rng.gen_range(0, self.start_words.len());
            self.start_words[num].clone()
        }

        fn check_punctuation(&self, inString: &str, outVec: &Vec::<String>) {
            ()
        }

        pub fn get_name(&self) -> String {
            self.name.clone()
        }
    }

}
