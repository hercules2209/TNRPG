
use std::io;

// Constants should be in uppercase.
const HEALTH_MAX: i16 = 100;
const DAMAGE: i16 = 15;
// const MANA_MAX: i16 =100;
// const STAMINA_MAX:i16=100;
fn main() {
    // Initialize health with the constant value.
    let mut health = HEALTH_MAX;
    let mut k = 1; // Set k to 1 to enter the loop.

    while k != 0 {
        println!("What would you like to do? \n Enter 1 to heal\n 2 to call crysis\n3 to slash\n 4 to run \n 5 to exit");
        let mut user_input = String::new(); // Declare user_input variable.
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        // Parse the input inside the match to handle it properly.
        let user_input: i8 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue; // Skip the rest of the loop if input is invalid.
            }
        };

        match user_input {
            1 => health_fn(3, &mut health), 
            2 => crysis(3, &mut health),
            3 => slash(&mut health),
            4 => run(&mut health),
            5 => k -= 1, // Decrement k to exit the loop.
            _ => println!("Unrecognized Statement"),
        }
    }

    println!("Your final Health was {}\nExiting the game...\nBye", health);
}

fn crysis(sec: i8, health: &mut i16) {
    for _ in 0..sec {
        *health -= DAMAGE;
    };
    println!("You took damage.\n Your health is now {}", *health);
}

fn health_fn(sec: i8, health: &mut i16) { // Renamed to avoid conflict with keyword.
    for _ in 0..sec {
        *health += (DAMAGE as f32 * 0.8) as i16; // Changed to increase health.
    }
    println!("You took three seconds to heal.\nYour health now is {}", *health);
}

fn run(health: &mut i16) {
    match *health {
        80..=100 => println!("You managed to escape"),
        40..=79 => println!("You ran away like a coward with your tail tucked in."),
        0..=39 => println!("You failed to escape"),
        _ => println!("Invalid health value"),
    }
}

fn slash(health: &mut i16) {
    *health -= DAMAGE;
}

