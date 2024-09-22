use io::{platform::unix::*, EventHandlerTrait, Event};

fn main() {
    let event_handler = EventHandler::init();

    loop {
        let event = event_handler.get_latest_event();

        match event {
            Ok(event) => {
                match event {
                    Some(event) => {
                        match event {
                            Event::Mouse(modifier, mouse_event) => println!("Modifier: {:?} | MouseEvent: {:?}", modifier, mouse_event),
                            Event::Character(character) => println!("Typed character: {character}"),
                        }
                    }
                    None => break,
                }
            }
            Err(msg) => println!("Error: {msg}"),
        }
    }
}