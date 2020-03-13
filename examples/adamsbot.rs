extern crate textsim;
use textsim::textsim::Model;
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
extern crate serde_json;
//use serde::Deserialize;

fn main() {
    let mut models = HashMap::<String, Model>::new();
    let models_file_path = Path::new("models.json");
    if let Ok(models_file) = File::open(models_file_path){
        let reader = BufReader::new(models_file);
        models = match serde_json::from_reader::<BufReader<File>, HashMap<String, Model>>(reader){
            Ok(v) => v,
            Err(_) => HashMap::<String, Model>::new()
        };
    }
    if models.is_empty(){
        let modl = Model::new("Adams".to_string(), 2);
        models.insert(modl.get_name(), modl);
        models.get_mut("Adams")
              .unwrap()
              .process_text(Path::new("1825-Adams.txt"));
        models.get_mut("Adams")
              .unwrap()
              .process_text(Path::new("1797-Adams.txt"));
    }
    
    let text = models.get_mut("Adams")
                     .unwrap()
                     .generate_text(25);
    println!("{:?}", &text);
    let file = File::create("models.json")
                    .expect("No such file");
    serde_json::to_writer(file, &models)
                .expect("Could not create writer");
}