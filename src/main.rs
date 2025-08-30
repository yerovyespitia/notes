use std::io;
use std::fs::File;
use std::fs::exists;
use std::fs::OpenOptions;
use std::io::Write;
use inline_colorization::*;

fn main() {

  loop {
    println!("\nWhat do you want to do today? (add, list, remove or quit)");

    let mut commands = String::from("");

    io::stdin().read_line(&mut commands).expect("Failed to read your commands");

    match commands.trim() {
      "add" => {
        add_note();
      }
      "list" => {
        println!("You chose to list notes.");
      }
      "remove" => {
        println!("You chose to remove a note.");
      }
      "quit" => {
        println!("\n{color_green}Exiting...{color_reset}");
        break;
      }
      _ => println!("\n{color_red}Invalid command. Please enter add, list, remove or quit.{color_reset}")
    };
  }
}

fn add_note() {
  let mut note = String::new();
  println!("\nWhat are you thinking?: ");
  io::stdin().read_line(&mut note).expect("Failed to read note");

  if exists("note.txt").expect("Could not check if file exists") {
    let mut file = OpenOptions::new()
      .append(true)
      .open("note.txt")
      .expect("Could not open file");

    file.write_all(note.as_bytes()).expect("Could not write to file");

    println!("\n{color_green}Note successfully added.{color_reset}");
  } else {
    let mut file = File::create("note.txt").expect("Could not create file");
    file.write_all(note.as_bytes()).expect("Could not write to file");

    println!("\n{color_green}Note successfully added.{color_reset}");
  }
}
