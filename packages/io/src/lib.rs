mod ansi_interpretor;
pub mod platform;

#[derive(Debug, Clone)]
pub enum Modifier {
    None,
    Shift,
    Ctrl,
    Alt,
}

#[derive(Debug, Clone)]
pub enum Motion {
    Down,
    Up,
    Move,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Down,
    Up,
}

#[derive(Debug)]
pub enum MouseEvent {
    Left(Motion, u32, u32),
    Middle(Motion, u32, u32),
    Right(Motion, u32, u32),
    Scroll(Direction),
}

pub(crate) enum MouseButton {
    Left,
    Middle,
    Right,
    Scroll,
}

#[derive(Default)]
pub(crate) struct MouseEventBuilder {
    motion: Option<Motion>,
    x: Option<u32>,
    y: Option<u32>,
    direction: Option<Direction>,
    button: Option<MouseButton>,
}

impl MouseEventBuilder {
    fn build(&self) -> MouseEvent {
        match self.button.as_ref().unwrap() {
            MouseButton::Left => MouseEvent::Left(self.motion.as_ref().unwrap().clone(), self.x.unwrap(), self.y.unwrap()),
            MouseButton::Middle => MouseEvent::Middle(self.motion.as_ref().unwrap().clone(), self.x.unwrap(), self.y.unwrap()),
            MouseButton::Right => MouseEvent::Right(self.motion.as_ref().unwrap().clone(), self.x.unwrap(), self.y.unwrap()),
            MouseButton::Scroll => MouseEvent::Scroll(self.direction.as_ref().unwrap().clone())
        }
    }
}

pub enum Event {
    Mouse(Modifier, MouseEvent),
    Character(char),
}

pub trait EventHandlerTrait {
    fn init() -> Self;
    fn get_latest_event(&self) -> Result<Option<Event>, &'static str>;
}
