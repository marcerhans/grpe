use io::{platform::unix::*, EventHandlerTrait};

fn main() {
    let blocking = false;
    let event_handler = EventHandler::init().expect("Failed to initialize event handler.");

    while event_handler.running() {
        let character = event_handler.latest_character(blocking);

        match character {
            Ok(character) => {
                println!("Typed character: {character}");
            }
            Err(msg) => if blocking {
                println!("{msg}");
            } else {
                // Do nothing, will be too much too print!
            }
        }
    }

    println!("IO handler stopped running.");
}