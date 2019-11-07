use ggez::*;
use super::core::Updatable;
use super::core::Clock;

struct MouseListener {
    mouse: input::mouse::MouseContext,
    event_list: Vec<Box<dyn Fn() -> i32>>,
}

impl MouseListener {
    
}

impl Updatable for MouseListener {
    fn update(t: Clock) -> Result<(), &'static str> {
        Ok(())
    }
}
