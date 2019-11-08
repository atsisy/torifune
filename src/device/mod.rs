use ggez::*;
use super::core::Updatable;
use super::core::Clock;
use ggez::input::mouse::MouseButton;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use super::numeric;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MouseButtonStatus {
    MOUSE_PRESSED,
    MOUSE_RELEASED,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MouseButtonEvent {
    Clicked,
    Pressed,
}

pub struct MouseListener {
    last_clicked: numeric::Vector2f,
    button_map: HashMap<MouseButton, MouseButtonStatus>,
    event_handlers: HashMap<MouseButton, HashMap<MouseButtonEvent, Vec<Box<dyn Fn() -> i32>>>>,
}

impl MouseListener {

    pub fn new() -> MouseListener {
        let mut button_map = HashMap::new();
        button_map.insert(MouseButton::Left, MouseButtonStatus::MOUSE_RELEASED);
        button_map.insert(MouseButton::Middle, MouseButtonStatus::MOUSE_RELEASED);
        button_map.insert(MouseButton::Right, MouseButtonStatus::MOUSE_RELEASED);

        let mut events = HashMap::new();
        let mut tmp = HashMap::new();
        tmp.insert(MouseButtonEvent::Clicked, Vec::new());
        tmp.insert(MouseButtonEvent::Pressed, Vec::new());
        events.insert(MouseButton::Left, tmp);

        let mut tmp = HashMap::new();
        tmp.insert(MouseButtonEvent::Clicked, Vec::new());
        tmp.insert(MouseButtonEvent::Pressed, Vec::new());
        events.insert(MouseButton::Middle, tmp);

        let mut tmp = HashMap::new();
        tmp.insert(MouseButtonEvent::Clicked, Vec::new());
        tmp.insert(MouseButtonEvent::Pressed, Vec::new());
        events.insert(MouseButton::Right, tmp);
        
        MouseListener {
            last_clicked: numeric::Vector2f{x: 0.0, y: 0.0},
            button_map: button_map,
            event_handlers: events,
        }
    }

    pub fn register_event_handler<F>(&mut self, button: MouseButton, event: MouseButtonEvent, f: &'static F)
    where F: Fn() -> i32 {
        self.event_handlers
            .get_mut(&button)
            .unwrap()
            .get_mut(&event)
            .unwrap()
            .push(Box::new(f));
    }
    
    #[inline(always)]
    pub fn get_position(&self, ctx: &ggez::Context) -> numeric::Point2f {
        input::mouse::position(ctx)
    }

    fn check_button(ctx: &ggez::Context, button: MouseButton) -> MouseButtonStatus {
        if input::mouse::button_pressed(ctx, MouseButton::Left) {
            MouseButtonStatus::MOUSE_PRESSED
        } else {
            MouseButtonStatus::MOUSE_RELEASED
        }
    }

    fn __flush_button_event(&mut self, button: MouseButton, current_state: &MouseButtonStatus) {
        if *current_state != self.button_map[&button] {
            let event = match *current_state {
                MouseButtonStatus::MOUSE_PRESSED => MouseButtonEvent::Pressed,
                MouseButtonStatus::MOUSE_RELEASED => MouseButtonEvent::Clicked,
            };
            for f in &self.event_handlers[&button][&event] {
                f();
            }
        }
    }

    fn flush_button_event(&mut self,
                          l_state: &MouseButtonStatus,
                          m_state: &MouseButtonStatus,
                          r_state: &MouseButtonStatus) {
        self.__flush_button_event(MouseButton::Left, l_state);
        self.__flush_button_event(MouseButton::Middle, m_state);
        self.__flush_button_event(MouseButton::Right, r_state);
    }
}

impl Updatable for MouseListener {
    fn update(&mut self, ctx: &ggez::Context, t: Clock) -> Result<(), &'static str> {
        let (l_status, m_status, r_status) = (
            MouseListener::check_button(ctx, MouseButton::Left),
            MouseListener::check_button(ctx, MouseButton::Middle),
            MouseListener::check_button(ctx, MouseButton::Right)
        );

        self.flush_button_event(&l_status, &m_status, &r_status);

        self.button_map.insert(MouseButton::Left, l_status);
        self.button_map.insert(MouseButton::Middle, m_status);
        self.button_map.insert(MouseButton::Right, r_status);
        
        Ok(())
    }
}
