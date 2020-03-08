extern crate textsim;
use textsim::*;
use std::path::Path;

fn main() {
    let mut modl = textsim::textsim::Model::new("Adams".to_string(), 2);
    modl.process_text(Path::new("1825-Adams.txt"));
    modl.process_text(Path::new("1797-Adams.txt"));
    let text = modl.generate_text(25);
    println!("{:?}", &text);
}

/*fn menu_main() {
    println!("RoboText Main Menu\n");
    println!("Select an Option");
    println!("(1) Create New Model");
    println!("(2) Load Saved Model");
    println!("(3) Delete Model");
    println!("(4) Quit");
}

fn create_model() {
    println!("Enter a name for your Model");
    

}*/