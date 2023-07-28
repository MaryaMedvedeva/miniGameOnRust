use std::io;

struct Human {
    name: String,
    age: i32,
    pet: Animal,
}

struct Animal {
    species: String,
    name: String,
    hunger: i32,
    boredom: i32,
}

impl Human {
    fn new(name: String, age: i32, pet_species: String, pet_name: String) -> Human {
        let pet = Animal {
            species: pet_species,
            name: pet_name,
            hunger: 0,
            boredom: 0,
        };
        Human {
            name,
            age,
            pet,
        }
    }

    fn speak(&self, message: &str) {
        println!("{}: {}", self.name, message);
    }

    fn introduce(&self) {
        println!("Привіт! Мене звуть {}. Мені {} років.", self.name, self.age);
        println!("У мене є тваринний друг. Його/її звуть {}, і це {}.", self.pet.name, self.pet.species);
    }

    fn interact_with_pet(&mut self) {
        println!("{}, у вас є {} з ім'ям {}.", self.name, self.pet.species, self.pet.name);
        for i in 0..6 {
            println!("Раунд {}: виберіть дію для взаємодії з твариною:", i + 1);
            println!("1 - погодувати");
            println!("2 - пограти");
            println!("3 - відпочити");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let action: &str = input.trim();

            if action == "1" {
                self.pet.react_to_action("погодувати");
            } else if action == "2" {
                self.pet.react_to_action("пограти");
            } else if action == "3" {
                self.pet.react_to_action("відпочити");
            } else {
                println!("Недопустима дія! Спробуйте ще раз.");
            }

            if self.pet.hunger == 2 && self.pet.boredom == 2 {
                println!("{} переїла і втомилася. Ви програли!", self.pet.name);
                return;
            } else if self.pet.hunger == 2 {
                println!("{} переїла. Ви програли!", self.pet.name);
                return;
            } else if self.pet.boredom == 2 {
                println!("{} втомилася. Ви програли!", self.pet.name);
                return;
            } else if i == 5 {
                println!("Ви виграли!");
            }

            if i < 5 {
                println!("Залишилося спроб: {}", 5 - i - 1);
            }
        }
    }
}

impl Animal {
    fn react_to_action(&mut self, action: &str) {
        match action {
            "погодувати" => {
                self.hunger -= 1;
                println!("{} покуштував.", self.name);
            }
            "пограти" => {
                self.boredom -= 1;
                println!("{} погрався.", self.name);
            }
            "відпочити" => {
                self.hunger += 1;
                self.boredom += 1;
                println!("{} відпочив.", self.name);
            }
            _ => {
                println!("Недопустима дія! Спробуйте ще раз.");
            }
        }
    }
}

fn story_telling() {
    println!("Введіть ім'я першої дійової особи:");
    let mut player1_name = String::new();
    io::stdin().read_line(&mut player1_name).expect("Failed to read line");
    let player1_name = player1_name.trim().to_string();

    println!("Введіть вік першої дійової особи:");
    let mut player1_age = String::new();
    io::stdin().read_line(&mut player1_age).expect("Failed to read line");
    let player1_age: i32 = player1_age.trim().parse().expect("Please enter a valid age.");

    println!("Введіть вид тварини для першої дійової особи (Кішка, Собака, тощо):");
    let mut player1_pet_species = String::new();
    io::stdin().read_line(&mut player1_pet_species).expect("Failed to read line");
    let player1_pet_species = player1_pet_species.trim().to_string();

    println!("Введіть ім'я тварини для першої дійової особи:");
    let mut player1_pet_name = String::new();
    io::stdin().read_line(&mut player1_pet_name).expect("Failed to read line");
    let player1_pet_name = player1_pet_name.trim().to_string();

    println!("Введіть ім'я другої дійової особи:");
    let mut player2_name = String::new();
    io::stdin().read_line(&mut player2_name).expect("Failed to read line");
    let player2_name = player2_name.trim().to_string();

    println!("Введіть вік другої дійової особи:");
    let mut player2_age = String::new();
    io::stdin().read_line(&mut player2_age).expect("Failed to read line");
    let player2_age: i32 = player2_age.trim().parse().expect("Please enter a valid age.");

    println!("Введіть вид тварини для другої дійової особи (Кішка, Собака, тощо):");
    let mut player2_pet_species = String::new();
    io::stdin().read_line(&mut player2_pet_species).expect("Failed to read line");
    let player2_pet_species = player2_pet_species.trim().to_string();

    println!("Введіть ім'я тварини для другої дійової особи:");
    let mut player2_pet_name = String::new();
    io::stdin().read_line(&mut player2_pet_name).expect("Failed to read line");
    let player2_pet_name = player2_pet_name.trim().to_string();

    let mut player1 = Human::new(player1_name, player1_age, player1_pet_species, player1_pet_name);
    let mut player2 = Human::new(player2_name, player2_age, player2_pet_species, player2_pet_name);

    println!("----- Зустріч -----");
    player1.introduce();
    player2.introduce();

    println!("----- Діалог -----");
    player1.speak(&format!("Привіт! Мене звати {}. Мені {} років. А тебе як звати?", player1.name, player1.age));
    player2.speak(&format!("Привіт! Мене зовуть {}. Мені {} років. Дуже приємно познайомитися!", player2.name, player2.age));
    player1.speak("Також приємно познайомитися!");

    println!("----- Взаємодія з твариною -----");
    player1.interact_with_pet();
    player2.interact_with_pet();
}

fn main() {
    story_telling();
}
