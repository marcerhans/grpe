use io::{platform::unix::*, EventHandlerTrait};

fn main() {
    let event_handler = EventHandler::init();

    while event_handler.running() {
        let character = event_handler.latest_character();

        match character {
            Ok(character) => {
                println!("Typed character: {character}");
            }
            Err(msg) => println!("Error: {msg}"),
        }
    }

    println!("IO handler stopped running.");
}