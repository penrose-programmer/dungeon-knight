use std::io;

struct Player {
    level: u32,
    health: u32,
}

fn main() {
    let player = Player { level: 1, health: 10 };

    loop {
        info(player.level, player.health);
        monster();
    }
}

fn info(level: u32, health: u32) {
    println!("You are on level {level}");
    println!("Your health is {health}/10");
}

fn encounter() {

    let mut choice = String::new();

    println!("You see a monster!");
    println!("You have three options:");
    println!("1. Fight it!");
    println!("2. Sneak past...");
    println!("3. Surrender!");
    println!("What do you choose? (1, 2, or 3)");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let mut choice = choice.trim().parse().expect("Please type a number!");

    match choice {
        1 => fight(),
        2 => sneak(),
        3 => surrender(),
        _ => println!("Your head becomes dizzy. You have to make a choice!")
    }
}

fn fight() {
    println!("You chose to fight!");
}

fn sneak() {
    println!("You chose to sneak past!");
}

fn surrender() {
    println!("You chose to surrender!");
}
