use ggez::*;
use ggez::input::mouse::MouseButton;
use ggez::input;
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
    last_clicked: HashMap<MouseButton, numeric::Point2f>,
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
            last_clicked: hash![
                (MouseButton::Left, numeric::Point2f { x: 0.0, y: 0.0 }),
                (MouseButton::Middle, numeric::Point2f { x: 0.0, y: 0.0 }),
                (MouseButton::Right, numeric::Point2f  {x: 0.0, y: 0.0 })
            ],
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

    //
    // 最後のクリック座標を返すメソッド
    //
    pub fn get_last_clicked(&self, button: MouseButton) -> numeric::Point2f {
        match button {
            MouseButton::Left | MouseButton::Middle | MouseButton::Right => self.last_clicked[&button],
            _ => panic!("Other MouseButton is detected!!"),
        }
    }

    fn __flush_button_event(&mut self, ctx: &ggez::Context, button: MouseButton, current_state: &MouseButtonStatus) {
        // 入力内容が以前と異なる
        if *current_state != self.button_map[&button] {
            
            // 操作を検知
            let event = match *current_state {
                MouseButtonStatus::MousePressed => MouseButtonEvent::Pressed,
                MouseButtonStatus::MouseReleased => {

                    // clickされた場合、last_clickにセット
                    self.last_clicked.insert(button, self.get_position(ctx));
                    
                    MouseButtonEvent::Clicked
                },
            };

            // ボタン・操作の情報を利用してクロージャのリストの要素を全て実行
            for f in &self.event_handlers[&button][&event] {
                f();
            }
        }
    }

    fn flush_button_event(&mut self,
                          ctx: &ggez::Context,
                          l_state: &MouseButtonStatus,
                          m_state: &MouseButtonStatus,
                          r_state: &MouseButtonStatus) {
        self.__flush_button_event(ctx, MouseButton::Left, l_state);
        self.__flush_button_event(ctx, MouseButton::Middle, m_state);
        self.__flush_button_event(ctx, MouseButton::Right, r_state);
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
        self.flush_button_event(ctx, &l_status, &m_status, &r_status);

        self.button_map.insert(MouseButton::Left, l_status);
        self.button_map.insert(MouseButton::Middle, m_status);
        self.button_map.insert(MouseButton::Right, r_status);
        
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum VirtualKey {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
    LeftSub = 4,
    RightSub = 5,
    UpSub = 6,
    DownSub = 7,
    LeftSubSub = 8,
    RightSubSub = 9,
    UpSubSub = 10,
    DownSubSub = 11,
    Action1 = 12,
    Action2 = 13,
    Action3 = 14,
    Action4 = 15,
    Action5 = 16,
    Action6 = 17,
    Action7 = 18,
    Action8 = 19,
    Mod1 = 20,
    Mod2 = 21,
    Mod3 = 22,
    Mod4 = 23,
    Unknown = 24,
}

impl VirtualKey {
    fn from_i32(i: i32) -> VirtualKey {
        match i {
            0 => VirtualKey::Left,
            1 => VirtualKey::Right,
            2 => VirtualKey::Up,
            3 => VirtualKey::Down,
            4 => VirtualKey::LeftSub,
            5 => VirtualKey::RightSub,
            6 => VirtualKey::UpSub,
            7 => VirtualKey::DownSub,
            8 => VirtualKey::LeftSubSub,
            9 => VirtualKey::RightSubSub,
            10 => VirtualKey::UpSubSub,
            11 => VirtualKey::DownSubSub,
            12 => VirtualKey::Action1,
            13 => VirtualKey::Action2,
            14 => VirtualKey::Action3,
            15 => VirtualKey::Action4,
            16 => VirtualKey::Action5,
            17 => VirtualKey::Action6,
            18 => VirtualKey::Action7,
            19 => VirtualKey::Action8,
            20 => VirtualKey::Mod1,
            21 => VirtualKey::Mod2,
            22 => VirtualKey::Mod3,
            23 => VirtualKey::Mod4,
            _ => VirtualKey::Unknown,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum KeyStatus {
    Pressed,
    Released,
    Unknown,
}

impl KeyStatus {
    
    #[inline(always)]
    fn positive_logic(b: bool) -> KeyStatus {
        if b {
            KeyStatus::Pressed
        } else {
            KeyStatus::Released
        }
    }

    #[inline(always)]
    fn negative_logic(b: bool) -> KeyStatus {
        if b {
            KeyStatus::Released
        } else {
            KeyStatus::Pressed
        }
    }

    
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum KeyboardEvent {
    Typed,
    FirstPressed,
    KeepPressed,
    KeepReleased,
    NumberOfKeyboardEvent,
    Unknown,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum KeyInputDevice {
    GenericKeyboard,
    PS3Controller,
}


fn vkey_input_check_generic_keyboard(ctx: &Context, vkey: &VirtualKey) -> KeyStatus {
    KeyStatus::positive_logic(
        match vkey {
            VirtualKey::Left => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Left),
            VirtualKey::Right => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Right),
            VirtualKey::Up => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Up),
            VirtualKey::Down => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Down),
            VirtualKey::LeftSub => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::A),
            VirtualKey::RightSub => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::D),
            VirtualKey::UpSub => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::W),
            VirtualKey::DownSub => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::S),
            VirtualKey::LeftSubSub => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::J),
            VirtualKey::RightSubSub => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::L),
            VirtualKey::UpSubSub => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::I),
            VirtualKey::DownSubSub => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::K),
            VirtualKey::Action1 => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Z),
            VirtualKey::Action2 => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::X),
            VirtualKey::Action3 => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::C),
            VirtualKey::Action4 => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::V),
            VirtualKey::Action5 => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::N),
            VirtualKey::Action6 => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::M),
            VirtualKey::Action7 => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Comma),
            VirtualKey::Action8 => input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Period),
            VirtualKey::Mod1 => input::keyboard::is_mod_active(ctx, input::keyboard::KeyMods::SHIFT),
            VirtualKey::Mod2 => input::keyboard::is_mod_active(ctx, input::keyboard::KeyMods::CTRL),
            VirtualKey::Mod3 => input::keyboard::is_mod_active(ctx, input::keyboard::KeyMods::ALT),
            VirtualKey::Mod4 => input::keyboard::is_mod_active(ctx, input::keyboard::KeyMods::LOGO),
            _ => false,
        }       
    )
}

fn vkey_input_check_not_implemented(_ctx: &Context, _vkey: &VirtualKey) -> KeyStatus {
    println!("device handler is not Implemented!!");
    KeyStatus::Unknown
}

fn vkey_input_check(ctx: &Context, device: &KeyInputDevice, vkey: &VirtualKey) -> KeyStatus {
    match device {
        &KeyInputDevice::GenericKeyboard => vkey_input_check_generic_keyboard(ctx, vkey),
        &KeyInputDevice::PS3Controller => vkey_input_check_not_implemented(ctx, vkey),
    }
}

///
/// # キーボードの状態を監視する構造体
/// イベントハンドラを登録し、呼び出すことが出来る
///
/// ## フィールド
/// ### devices
/// 監視するデバイスのベクタ
///
/// ### listening
/// 監視するVirtualKeyのベクタ
/// newで生成すると全てのVirtualKeyを監視するようになる。 new_maskedで生成すると監視するVirtualKeyを設定でき、
/// コストが低下する。
///
/// ### key_map
/// key_mapはVec<KeyStatus>型である。KeyStatusをusizeにキャストしてアドレッシングするため、HashMap型ではない
///
/// ### event_handlers
/// event_handlers[VirtualKey][KeyStatus]  ====>  クロージャのベクタ
///
pub struct KeyboardListener {
    devices: Vec<KeyInputDevice>,
    listening: Vec<VirtualKey>,
    key_map: Vec<KeyStatus>,
    event_handlers: Vec<Vec<Vec<Box<dyn Fn() -> i32>>>>,
}

impl KeyboardListener {

    /// # ScheduledEvent構造体の生成メソッド 
    pub fn new(devices: Vec<KeyInputDevice>) -> KeyboardListener {
        // key_mapは全てReleasedで初期化
        let mut key_map = vec![KeyStatus::Released; (VirtualKey::Unknown as usize) + 1];
        let mut listening = Vec::new();

        let mut events: Vec<Vec<Vec<Box<dyn Fn() -> i32>>>> = Vec::new();
        for vkey_raw in 0..(VirtualKey::Unknown as i32 + 1) {
            let mut tmp: Vec<Vec<Box<dyn Fn() -> i32>>> = Vec::new();
            for _ in 0..(KeyboardEvent::Unknown as i32 + 1) {
                tmp.push(Vec::new());
            }
            events.push(tmp);

            // ListeningするVirtualKeyは全て
            listening.push(VirtualKey::from_i32(vkey_raw));
        }
        
        KeyboardListener {
            devices: devices,
            listening: listening,
            key_map: key_map,
            event_handlers: events,
        }
    }

    ///
    /// # ScheduledEvent構造体の生成メソッド
    ///
    pub fn new_masked(devices: Vec<KeyInputDevice>, listening: Vec<VirtualKey>) -> KeyboardListener {
        // key_mapは全てReleasedで初期化
        let key_map = vec![KeyStatus::Released; (VirtualKey::Unknown as usize) + 1];

        let mut events: Vec<Vec<Vec<Box<dyn Fn() -> i32>>>> = Vec::new();
        for _ in 0..(VirtualKey::Unknown as i32 + 1) {
            let mut tmp: Vec<Vec<Box<dyn Fn() -> i32>>> = Vec::new();
            for _ in 0..(KeyboardEvent::Unknown as i32 + 1) {
                tmp.push(Vec::new());
            }
            events.push(tmp);
        }
        
        KeyboardListener {
            devices: devices,
            listening: listening,
            key_map: key_map,
            event_handlers: events,
        }
    }
    
    ///
    /// キーボードのイベントハンドラを登録するためのメソッド
    ///
    pub fn register_event_handler<F>(&mut self, key: VirtualKey, event: KeyboardEvent, f: &'static F)
    where F: Fn() -> i32 {
        self.event_handlers
            .get_mut(key as usize)
            .unwrap()
            .get_mut(event as usize)
            .unwrap()
            .push(Box::new(f));
    }

    ///
    /// キー入力に応じてイベントハンドラを呼び出すメソッド
    ///
    fn flush_key_event(&self, ctx: &ggez::Context, t: Clock, vkey: &VirtualKey, current_state: &KeyStatus) {
        let event = if *current_state != *self.key_map.get(*vkey as usize).unwrap() {
            match current_state {
                &KeyStatus::Pressed => KeyboardEvent::FirstPressed,
                &KeyStatus::Released => KeyboardEvent::Typed,
                _ => KeyboardEvent::Unknown,
            }
        } else {
            match current_state {
                &KeyStatus::Pressed => KeyboardEvent::KeepPressed,
                &KeyStatus::Released => KeyboardEvent::KeepReleased,
                _ => KeyboardEvent::Unknown,
            }
        };

        for f in self.event_handlers
            .get(*vkey as usize)
            .unwrap()
            .get(event as usize)
            .unwrap() {
            f();
        }
        
    }

    ///
    /// 複数のキー入力デバイスの状態をミックスするメソッド
    /// 基本的に内部メソッドとして利用する
    ///
    fn current_key_status(&self, ctx: &ggez::Context, vkey: &VirtualKey) -> KeyStatus {
        
        for device in &self.devices {
            if vkey_input_check(ctx, device, vkey) == KeyStatus::Pressed {
                return KeyStatus::Pressed;
            }
        }

        KeyStatus::Released
    }

}

impl Updatable for KeyboardListener {
    fn update(&mut self, ctx: &ggez::Context, t: Clock) -> Result<(), &'static str> {

        for vkey in &self.listening {
            let current_state = self.current_key_status(ctx, vkey);
            self.flush_key_event(ctx, t, &vkey, &current_state);
            self.key_map[*vkey as usize] = current_state;
        }
        
        Ok(())
    }
}
