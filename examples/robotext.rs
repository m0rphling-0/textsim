extern crate textsim;
use textsim::textsim::Model;
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
extern crate serde_json;
use std::io::stdin;
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
    loop {
        menu_main();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Input read error");
        let input_match = match input.trim_end() {
            "1" => create_model(),
            "2" => load_model(),
            "3" => delete_model(),
            "4" => break
        };
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

fn menu_main() {
    println!("RoboText Main Menu\n");
    println!("Select an Option");
    println!("1 - Create New Model");
    println!("2 - Use Saved Model");
    println!("3 - Delete Model");
    println!("4 - Quit");
    println!("Enter option number");
}

/*fn create_model() -> Model {
    println!("Enter a name for your Model");

    
}*/