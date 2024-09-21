use io::{platform::unix::*, EventHandlerTrait, Event};

fn main() {
    let event_handler = EventHandler::init();

    loop {
        let event = event_handler.get_latest_event();

        match event {
            Some(event) => {
                match event {
                    Event::Mouse(mouse_event) => println!("mouse"),
                    Event::Letter(character) => println!("Typed character: {character}"),
                }
            }
            None => break,
        }
    }
}