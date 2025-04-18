mod ansi_interpretor;
pub mod platform;

#[derive(Debug, Clone)]
pub enum Modifier {
    None,
    Ctrl,
    Alt,
}

pub mod mouse {
    #[derive(Debug, Clone)]
    pub enum Motion {
        Down,
        Up,
    }

    #[derive(Debug, Clone)]
    pub enum Direction {
        Down,
        Up,
    }

    #[derive(Debug)]
    pub enum Event {
        Left(Motion, u32, u32),
        Middle(Motion, u32, u32),
        Right(Motion, u32, u32),
        Scroll(Direction),
    }

    pub(crate) enum Button {
        Left,
        Middle,
        Right,
        Scroll,
    }

    #[derive(Default)]
    pub(crate) struct EventBuilder {
        pub motion: Option<Motion>,
        pub x: Option<u32>,
        pub y: Option<u32>,
        pub direction: Option<Direction>,
        pub button: Option<Button>,
    }

    impl EventBuilder {
        pub fn build(self) -> Event {
            match self.button.unwrap() {
                Button::Left => Event::Left(
                    self.motion.unwrap(),
                    self.x.unwrap(),
                    self.y.unwrap(),
                ),
                Button::Middle => Event::Middle(
                    self.motion.unwrap(),
                    self.x.unwrap(),
                    self.y.unwrap(),
                ),
                Button::Right => Event::Right(
                    self.motion.unwrap(),
                    self.x.unwrap(),
                    self.y.unwrap(),
                ),
                Button::Scroll => Event::Scroll(self.direction.unwrap()),
            }
        }
    }
}

pub mod misc {
    pub struct CurrentSize(pub u64, pub u64);

    pub enum Event {
        CurrentSize(CurrentSize),
    }
}

pub enum Event {
    Mouse(Modifier, mouse::Event),
    Character(char),
    Misc(misc::Event),
}

pub trait EventHandlerTrait: Sized {
    fn init() -> Result<Self, &'static str>;
    fn running(&self) -> bool;
    fn latest_event(&self) -> Result<Event, &'static str>;
    fn latest_character(&self, blocking: bool) -> Result<char, &'static str>;
}
