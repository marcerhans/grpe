// use io::{platform::unix::*, EventHandlerTrait, Letter};
// use std::ffi::c_char;

// fn main() {
//     let event_handler = EventHandler::init();

//     loop {
//         match event_handler.receiver.recv().unwrap() {
//             io::Event::Mouse(_) => todo!(),
//             io::Event::Letter(c) => {
//                 println!("{}", c.0);
//             },
//         }
//     }
// }