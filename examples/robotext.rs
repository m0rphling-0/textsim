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
        let input = get_user_input();
        match input.trim_end() {
            "1" => {
                    if let Some(mut model) = create_model(&mut models){
                        use_model(&mut model);
                    }
            },
            "2" => load_and_use_model(&mut models),
            "3" => delete_model(&mut models),
            "4" => break,
            _   => println!("Invalid input. Choose 1, 2, 3, or 4")
        };
    }
    
    let file = File::create("models.json")
                    .expect("Could not create file");
    serde_json::to_writer(file, &models)
                .expect("Could not create writer. Alas, models were not saved :(");
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

fn create_model(models: &mut HashMap<String, Model>) -> Option<&mut Model> {
    println!("Enter a name for your Model:");
    let name = get_user_input();
    if models.contains_key(&name){
        println!("Model of that name already exists");
        return None
    }
    let mut attempt_count = 0;
    while attempt_count < 5 {
        println!("Enter the order of your Markov Chain Model as a positive integer:");
        let choice = get_user_input();
        if let Ok(num) = choice.parse::<usize>() {
            if num > 0 {
                let model = Model::new(name.to_string(), num);
                models.insert(name.to_owned(), model);
                return models.get_mut(&name)
            }
            else{
                attempt_count += 1;
                println!("Integer must be greater than 0");
                println!("{}, attempts left", 5 - attempt_count);
                continue
            }
        }
        else{
            attempt_count += 1;
            println!("Input could not be parsed as an integer");
            println!("{}, attempts left", 5 - attempt_count);
            continue
        }
    }
    None
}

fn load_and_use_model(models: &mut HashMap<String, Model>) -> () {
    list_models(models);
    let name = get_user_input();
    if models.contains_key(&name) {
        if let Some(model) = models.get_mut(&name){
            use_model(model);
        }
    }
    else {
        println!("There is no model named: {}", name);
    }
} 

fn delete_model(models: &mut HashMap<String, Model>) -> () {
    println!("Choose Model to delete:");
    list_models(models);
    let name = get_user_input();
    if models.contains_key(&name) {
        models.remove(&name);
    }
    else {
        println!("There is no model named: {}", name);
    }
}

fn list_models(models: &HashMap<String, Model>) -> () {
    println!("Available models:");
    for name in models.keys() {
        println!("{}", &name);
    }
}

fn use_model(model: &mut Model) -> (){
    loop {
        use_model_menu();
        let input = get_user_input();
        match input.as_ref() {
            "1" => train_model(model),
            "2" => user_generate_text(model),
            "3" => break,
            _   => println!("Invalid input. Choose 1, 2, 3")
        };
    }
}

fn train_model(model: &mut Model) {
    println!("Enter path of .txt file for training");
    println!("Path must be relative to project root");
    let input = get_user_input();
    let file_path = Path::new(&input);
    match model.process_text(file_path) {
        Ok(()) => println!("Model trained on file {},", input),
        Err(e) => println!("{}", e)
    }
}

fn user_generate_text(model: &mut Model) {
    println!("Enter word length of text to generate as an integer:");
    let choice = get_user_input();
        if let Ok(num) = choice.parse::<usize>() {
            if num > 0 {
                let text = model.generate_text(num);
                println!("{:?}", &text);
            }
            else{
                println!("Integer must be greater than 0");
            }
        }
        else{
            println!("Input could not be parsed as an integer");
        }
}

fn use_model_menu() -> () {
    println!("What would you like to do?");
    println!("1 - Train Model");
    println!("2 - Generate Text");
    println!("3 - Return to Main Menu");
}
    
fn get_user_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error reading input");
    input.trim().to_owned()
}
