use ggez::*;
use ggez::input::mouse::MouseButton;
use super::core::Updatable;
use super::core::Clock;
use std::collections::HashMap;
use std::hash::Hash;
use super::numeric;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum MouseButtonStatus {
    MousePressed,
    MouseReleased,
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

    /// # ScheduledEvent構造体の生成メソッド 
    pub fn new() -> MouseListener {
        let mut button_map = HashMap::new();
        
        button_map.insert(MouseButton::Left, MouseButtonStatus::MouseReleased);
        button_map.insert(MouseButton::Middle, MouseButtonStatus::MouseReleased);
        button_map.insert(MouseButton::Right, MouseButtonStatus::MouseReleased);

        let mut events = HashMap::new();
        events.insert(MouseButton::Left,
                      hash![
                          (MouseButtonEvent::Clicked, Vec::<Box<dyn Fn() -> i32>>::new()),
                          (MouseButtonEvent::Pressed, Vec::<Box<dyn Fn() -> i32>>::new())
                      ]);
        

        events.insert(MouseButton::Middle,
                      hash![
                          (MouseButtonEvent::Clicked, Vec::<Box<dyn Fn() -> i32>>::new()),
                          (MouseButtonEvent::Pressed, Vec::<Box<dyn Fn() -> i32>>::new())
                      ]);

        events.insert(MouseButton::Right,
                      hash![
                          (MouseButtonEvent::Clicked, Vec::<Box<dyn Fn() -> i32>>::new()),
                          (MouseButtonEvent::Pressed, Vec::<Box<dyn Fn() -> i32>>::new())
                      ]);
        
        MouseListener {
            last_clicked: numeric::Vector2f{x: 0.0, y: 0.0},
            button_map: button_map,
            event_handlers: events,
        }
    }

    ///
    /// マウスのイベントハンドラを登録するためのメソッド
    ///
    pub fn register_event_handler<F>(&mut self, button: MouseButton, event: MouseButtonEvent, f: &'static F)
    where F: Fn() -> i32 {
        self.event_handlers
            .get_mut(&button)
            .unwrap()
            .get_mut(&event)
            .unwrap()
            .push(Box::new(f));
    }

    //
    // 現在のマウスの座標を得るメソッド
    //
    #[inline(always)]
    pub fn get_position(&self, ctx: &ggez::Context) -> numeric::Point2f {
        input::mouse::position(ctx)
    }

    fn check_button(ctx: &ggez::Context, button: MouseButton) -> MouseButtonStatus {
        if input::mouse::button_pressed(ctx, MouseButton::Left) {
            MouseButtonStatus::MousePressed
        } else {
            MouseButtonStatus::MouseReleased
        }
    }

    fn __flush_button_event(&mut self, button: MouseButton, current_state: &MouseButtonStatus) {
        // 入力内容が以前と異なる
        if *current_state != self.button_map[&button] {
            
            // 操作を検知
            let event = match *current_state {
                MouseButtonStatus::MousePressed => MouseButtonEvent::Pressed,
                MouseButtonStatus::MouseReleased => MouseButtonEvent::Clicked,
            };

            // ボタン・操作の情報を利用してクロージャのリストの要素を全て実行
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

        //
        // 入力のイベントハンドラを実行する
        //
        self.flush_button_event(&l_status, &m_status, &r_status);

        self.button_map.insert(MouseButton::Left, l_status);
        self.button_map.insert(MouseButton::Middle, m_status);
        self.button_map.insert(MouseButton::Right, r_status);
        
        Ok(())
    }
}
