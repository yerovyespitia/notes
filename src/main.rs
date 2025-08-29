use std::io;
use inline_colorization::*;

fn main() {

  loop {
    println!("\nWhat do you want to do today? (add, list, remove)");

    let mut commands = String::from("");

    io::stdin().read_line(&mut commands).expect("Failed to read your commands");

    match commands.trim() {
      "add" => {
        println!("You chose to add a note.");
        // Add note logic here
        add_note();
        break;
      }
      "list" => {
        println!("You chose to list notes.");
        // List notes logic here
        break;
      }
      "remove" => {
        println!("You chose to remove a note.");
        // Remove note logic here
        break;
      }
      _ => println!("\n{color_red}Invalid command. Please enter add, list, or remove.{color_reset}\n")
    };
  }
}

fn add_note() {
  // Receive note input from user
  let mut note = String::new();
  println!("Enter your note:");
  io::stdin().read_line(&mut note).expect("Failed to read note");
  println!("Your note: {note}")

  // Create file is not exists
  // Append note to file
  // Confirm note added
  // Return to main menu
}
