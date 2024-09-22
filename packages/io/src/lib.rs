pub mod platform;
mod ansi_interpretor;

pub enum Modifier {
    None,
    Shift,
    Ctrl,
    Alt,
}

pub enum Motion {
    Down,
    Up,
    Move,
}

pub enum MouseEvent {
    Left(Motion, u32, u32),
    Middle(Motion, u32, u32),
    RightDown(Motion, u32, u32),
    Scroll(Motion),
}

pub enum Event {
    Mouse(Modifier, MouseEvent),
    Character(char),
}

pub trait EventHandlerTrait {
    fn init() -> Self;
    fn get_latest_event(&self) -> Option<Event>;
}
