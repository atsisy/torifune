use ggez::*;
use ggez::input::mouse::MouseButton;
use super::core::Updatable;
use super::core::Clock;
use std::collections::HashMap;
use super::numeric;

#[derive(Debug, Eq, PartialEq)]
enum MouseButtonStatus {
    MOUSE_PRESSED,
    MOUSE_RELEASED,
}

struct MouseListener {
    last_clicked: numeric::Vector2f,
    button_map: HashMap<MouseButton, MouseButtonStatus>,
    event_handlers: HashMap<MouseButton, HashMap<MouseButtonStatus, Vec<fn() -> i32>>>,
}

impl MouseListener {

    pub fn new() -> MouseListener {
        let mut button_map = HashMap::new();
        button_map.insert(MouseButton::Left, MouseButtonStatus::MOUSE_RELEASED);
        button_map.insert(MouseButton::Middle, MouseButtonStatus::MOUSE_RELEASED);
        button_map.insert(MouseButton::Right, MouseButtonStatus::MOUSE_RELEASED);

        let mut events = HashMap::new();
        let tmp = HashMap::new();
        tmp.insert(MouseButtonStatus::MOUSE_PRESSED, Vec::new());
        tmp.insert(MouseButtonStatus::MOUSE_RELEASED, Vec::new());
        events.insert(MouseButton::Left, tmp);

        let tmp = HashMap::new();
        tmp.insert(MouseButtonStatus::MOUSE_PRESSED, Vec::new());
        tmp.insert(MouseButtonStatus::MOUSE_RELEASED, Vec::new());
        events.insert(MouseButton::Middle, Vec::new());

        let tmp = HashMap::new();
        tmp.insert(MouseButtonStatus::MOUSE_PRESSED, Vec::new());
        tmp.insert(MouseButtonStatus::MOUSE_RELEASED, Vec::new());
        events.insert(MouseButton::Right, Vec::new());
        
        MouseListener {
            last_clicked: numeric::Vector2f{x: 0.0, y: 0.0},
            button_map: button_map,
            event_handlers: events,
        }
    }
    
    #[inline(always)]
    pub fn get_position(&self, ctx: &ggez::Context) -> numeric::Point2f {
        input::mouse::position(ctx)
    }
}

impl Updatable for MouseListener {
    fn update(&mut self, ctx: &ggez::Context, t: Clock) -> Result<(), &'static str> {
        if input::mouse::button_pressed(ctx, MouseButton::Left) != self.button_map[&MouseButton::Left] {
            
        }
        Ok(())
    }
}
