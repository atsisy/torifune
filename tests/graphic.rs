use ggez::*;
use ggez::input::mouse::MouseButton;
use std::env;
use std::path;
use trojan::device;
use trojan::graphics::object as tobj;
use ggez::graphics as ggraphics;
use trojan::core::Updatable;
use trojan::graphics::object::*;

struct State<'a> {
    frames: usize,
    text: graphics::Text,
    mouse: device::MouseListener,
    key: device::KeyboardListener,
    image: tobj::UniTextureObject<'a>,
}

impl<'a> State<'a> {
    fn new(ctx: &mut Context, image: tobj::UniTextureObject<'a>) -> GameResult<State<'a>> {
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
            image: image,
        };

        s.mouse
            .register_event_handler(
                MouseButton::Left,
                device::MouseButtonEvent::Clicked,
                &move |ctx, t| { println!("Clicked"); Ok(()) });
        s.mouse
            .register_event_handler(
                MouseButton::Left,
                device::MouseButtonEvent::Pressed,
                &move |ctx, t| { println!("Pre"); Ok(()) });

        s.key
            .register_event_handler(
                device::VirtualKey::Action1,
                device::KeyboardEvent::FirstPressed,
                &move |ctx, t| { println!("Pre"); Ok(()) });

        s.key
            .register_event_handler(
                device::VirtualKey::Action2,
                device::KeyboardEvent::FirstPressed,
                &move |ctx, t| { println!("Pre"); Ok(()) });


        Ok(s)
    }
}

impl<'a> ggez::event::EventHandler for State<'a> {
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
        graphics::draw(ctx, &self.text,
                       graphics::DrawParam::default()
                       .dest(dest_point)
                       .scale(trojan::numeric::Point2f {x: 0.2, y: 0.5}))?;
        self.image.draw(ctx)?;
        self.image.set_alpha(0.1);
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

    let textures = vec![ggraphics::Image::new(ctx, "/ghost1.png").unwrap()];
    let image = tobj::UniTextureObject::new(
        &textures[0],
        trojan::numeric::Point2f { x: 0.0, y: 0.0 },
        trojan::numeric::Vector2f { x: 1.0, y: 1.0 },
        0.0,
        0
    );
    let state = &mut State::new(ctx, image).unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
