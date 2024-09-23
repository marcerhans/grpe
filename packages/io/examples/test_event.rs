use io::{platform::unix::*, EventHandlerTrait, Event};

fn main() {
    let blocking = false;
    let event_handler = EventHandler::init(blocking);

    while event_handler.running() {
        let event = event_handler.latest_event();

        match event {
            Ok(event) => {
                match event {
                    Event::Mouse(modifier, mouse_event) => println!("Modifier: {:?} | MouseEvent: {:?}", modifier, mouse_event),
                    Event::Character(character) => println!("Typed character: {character}"),
                }
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