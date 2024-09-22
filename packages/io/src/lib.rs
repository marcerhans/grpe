mod ansi_interpretor;
pub mod platform;

#[derive(Debug, Clone)]
pub enum Modifier {
    None,
    Shift,
    Ctrl,
    Alt,
}

pub(crate) mod mouse {
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
        pub fn build(&self) -> Event {
            match self.button.as_ref().unwrap() {
                Button::Left => Event::Left(
                    self.motion.as_ref().unwrap().clone(),
                    self.x.unwrap(),
                    self.y.unwrap(),
                ),
                Button::Middle => Event::Middle(
                    self.motion.as_ref().unwrap().clone(),
                    self.x.unwrap(),
                    self.y.unwrap(),
                ),
                Button::Right => Event::Right(
                    self.motion.as_ref().unwrap().clone(),
                    self.x.unwrap(),
                    self.y.unwrap(),
                ),
                Button::Scroll => Event::Scroll(self.direction.as_ref().unwrap().clone()),
            }
        }
    }
}

pub enum Event {
    Mouse(Modifier, mouse::Event),
    Character(char),
}

pub trait EventHandlerTrait {
    fn init() -> Self;
    fn get_latest_event(&self) -> Result<Option<Event>, &'static str>;
}
