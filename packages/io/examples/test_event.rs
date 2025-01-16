use io::{platform::unix::*, EventHandlerTrait, Event};

fn main() {
    let event_handler = EventHandler::init().expect("Failed to initialize event handler.");

    while event_handler.running() {
        let event = event_handler.latest_event();

        match event {
            Ok(event) => {
                match event {
                    Event::Mouse(modifier, mouse_event) => println!("Modifier: {:?} | MouseEvent: {:?}", modifier, mouse_event),
                    Event::Character(character) => println!("Typed character: {character}"),
                    Event::Misc(event) => match event {
                        io::misc::Event::CurrentSize(current_size) => println!("New terminal size: {}x{}", current_size.0, current_size.1),
                    }
                }
            }
            Err(msg) => (), // println!("{msg}") // Note: If printing the error without any sleep, it will be too much to print!
        }
    }

    println!("IO handler stopped running.");
}