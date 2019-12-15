// push test
mod model;
use crate::model::character;
use crate::model::gender;


fn main() {
    let character = character::Character::new(enter_name(), enter_gender());
    println!("{:#?}", &character);
    character.diplay();
}

fn enter_gender() -> gender::Gender {
    println!("Please enter your gender:");
    let mut input = String::new();

    loop {
        std::io::stdin().read_line(&mut input).expect("IOException: read line failed.");
        match input.trim().to_lowercase().as_str() {
            "male" => {
                return gender::Gender::Male;
            }

            "female" => {
                return gender::Gender::Female;
            }

            _ => {
                eprintln!("Wrong enter! please try again.");
                input.clear();
            }
        }
    }
}

fn enter_name() -> String {
    println!("Please enter your name：");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("IOException: read line failed.");
    name.trim().to_string().clone()
}
