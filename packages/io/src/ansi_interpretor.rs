use crate::{Event, MouseEvent};
use std::ffi::c_char;

pub mod ascii {
    #![allow(non_upper_case_globals)]
    pub const ESC: i8 = 0x1B; // ESCAPE
    pub const M: i8 = 0x4D; // DOWN
    pub const m: i8 = 0x6D; // RELEASE
    pub const SEMI_COLON: i8 = 0x3B; // ;
}

pub fn is_escape_sequence(c: c_char) -> bool {
    static START_0: i8 = 0x1B; // ESC
    return c as i8 == START_0;
}

pub fn is_csi_sequence(c: c_char) -> bool {
    static START_1: i8 = 0x5B; // [
    return c as i8 == START_1;
}

/// Reads csi sequence with given reader [Fn] and returns [Event] based on input if recognized.
pub fn read_csi_sequence<F: Fn() -> c_char>(reader: F) -> Option<Event> {
    let mut sequence = vec![];

    // Read to end of sequence ('m' or 'M')
    loop {
        sequence.push((reader)());

        let last = sequence.last().unwrap();
        if *last == ascii::m || *last == ascii::M {
            break;
        }
    }

    let items: Vec<&[i8]> = sequence.split(|c| *c == ascii::SEMI_COLON).collect();

    if items.len() != 3 {
        panic!("CSI Sequence badly formatted or parsed")
    }

    // Convert from chars to string to values.
    let button_chrs = &items[0][1..];
    let x_chrs = &items[1];
    let y_chrs = &items[2][..(items[2].len() - 1)];

    fn digits_to_num(digits: &[i8]) -> u32 {
        let mut num = 0;

        for digit in digits.iter() {
            num = 10 * num + ((*digit - 48) as u32); // -48 because of ascii
        }

        num
    }

    let button = digits_to_num(button_chrs);
    let x = digits_to_num(x_chrs);
    let y = digits_to_num(y_chrs);

    // 0 = MB1, 1 = MB3, 2 = MB2, 32 = MB1_DRAG, 33, MB3_DRAG, 34, MB4_DRAG
    let ret = if *sequence.last().unwrap() == ascii::M {
        match button {
            0 => Some(Event::Mouse(MouseEvent::LeftDown(x, y))),
            1 => Some(Event::Mouse(MouseEvent::MiddleDown(x, y))),
            2 => Some(Event::Mouse(MouseEvent::RightDown(x, y))),
            32 => Some(Event::Mouse(MouseEvent::LeftMove(x, y))),
            33 => Some(Event::Mouse(MouseEvent::MiddleMove(x, y))),
            34 => Some(Event::Mouse(MouseEvent::RightMove(x, y))),
            _ => None
        }
    } else {
        match button {
            0 => Some(Event::Mouse(MouseEvent::LeftUp(x, y))),
            1 => Some(Event::Mouse(MouseEvent::MiddleUp(x, y))),
            2 => Some(Event::Mouse(MouseEvent::RightUp(x, y))),
            _ => None
        }
    };
    // sequence.split

    //     // Down?
    //     if c >= 32 && c <= 34 { // MB1 = 32+0, MB2 = 32+1, MB3 = 32+2
    //         // Which button ?
    //         let button = c - 32;

    //         match button {
    //             0 => return 
    //             1 => return Some(Event::Mouse(Mouse::RightDown(x, y))),
    //             2 => return Some(Event::Mouse(Mouse::MiddleDown(x, y))),
    //             _ => return None,
    //         }
    //     }

    //     // Move?
    //     if c >= 64 && c <= 66 { // MB1DRAG = 32+32+0, MB2DRAG = 32+32+1, ...
    //         // Which button ?
    //         let button = c - 64;

    //         match button {
    //             0 => return Some(Event::Mouse(Mouse::LeftMove(x, y))),
    //             1 => return Some(Event::Mouse(Mouse::RightMove(x, y))),
    //             2 => return Some(Event::Mouse(Mouse::MiddleMove(x, y))),
    //             _ => return None,
    //         }
    //     }

    //     // Release?
    //     if c == 35 {
    //         return Some(Event::Mouse(Mouse::Up(x, y)));
    //     }
    // }

    ret
}
