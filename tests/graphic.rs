use ggez::*;
use ggez::input::mouse::MouseButton;
use std::env;
use std::path;
use trojan::device;
use trojan::core::Updatable;

struct State {
    frames: usize,
    text: graphics::Text,
    mouse: device::MouseListener,
    key: device::KeyboardListener,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let font = graphics::Font::new(ctx, "/azuki.ttf")?;
        let mut text = graphics::Text::new("Hello");
        text.set_font(font, graphics::Scale {x: 48.0, y: 48.0});
        
        let mut s = State {
            frames: 0,
            text: text,
            mouse: device::MouseListener::new(),
            key: device::KeyboardListener::new_masked(
                vec![device::KeyInputDevice::GenericKeyboard],
                vec![device::VirtualKey::Action1, device::VirtualKey::Action2]),
        };

        s.mouse
            .register_event_handler(
                MouseButton::Left,
                device::MouseButtonEvent::Clicked,
                &move || { println!("Clicked"); 10 });
        s.mouse
            .register_event_handler(
                MouseButton::Left,
                device::MouseButtonEvent::Pressed,
                &move || { println!("Pre"); 10 });

        s.key
            .register_event_handler(
                device::VirtualKey::Action1,
                device::KeyboardEvent::FirstPressed,
                &move || { println!("Pre"); 10 });

        s.key
            .register_event_handler(
                device::VirtualKey::Action2,
                device::KeyboardEvent::FirstPressed,
                &move || { println!("Pre"); 10 });


        Ok(s)
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.mouse.update(ctx, 0);
        self.key.update(ctx, 0);
        let p = self.mouse.get_last_clicked(MouseButton::Left);
        println!("{}, {}", p.x, p.y);
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 0.0].into());

        let offset = self.frames as f32 / 10.0;
        let dest_point = nalgebra::Point2::new(offset, offset);
        graphics::draw(ctx, &self.text, (dest_point,))?;
        graphics::present(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", timer::fps(ctx));
        }
        
        Ok(())
    }
}

#[test]
pub fn graphic_test() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("test", "akichi").add_resource_path(resource_dir)
        .conf(c)
        .build()
        .unwrap();
    let state = &mut State::new(ctx).unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
