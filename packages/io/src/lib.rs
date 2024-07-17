pub mod platform;

pub enum Mouse {
    Down(u32, u32),
    Move(u32, u32),
    Up(u32, u32),
}

pub struct Letter(pub char);

pub enum Event {
    Mouse(Mouse),
    Letter(Letter),
}

/// Using trait as interface. What are you gonna do about it :)?
pub trait EventHandlerTrait {
    fn init() -> Self;
}
