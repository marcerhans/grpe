pub mod platform;
mod ansi_interpretor;

pub enum Mouse {
    LeftDown(u32, u32),
    LeftMove(u32, u32),
    LeftUp(u32, u32),
    MiddleDown(u32, u32),
    MiddleMove(u32, u32),
    MiddleUp(u32, u32),
    RightDown(u32, u32),
    RightMove(u32, u32),
    RightUp(u32, u32),
}

pub enum Event {
    Mouse(Mouse),
    Letter(char),
}

pub trait EventHandlerTrait {
    fn init() -> Self;
    fn get_latest_event(&self) -> Option<Event>;
}
