use ggez::*;
use ggez::input::mouse::MouseButton;
use std::env;
use std::path;
use trojan::device;
use trojan::graphics::object as tobj;
use ggez::graphics as ggraphics;
use trojan::core::Updatable;
use trojan::graphics::object::*;
use trojan::core::Clock;

struct State<'a> {
    frames: usize,
    text: SimpleText,
    mouse: device::MouseListener,
    key: device::KeyboardListener,
    image: tobj::SimpleObject<'a>,
}

fn sample_mouse_closure(msg: &'static str) -> Box<dyn Fn(&ggez::Context, Clock) -> Result<(), String>> {
    Box::new(move |ctx: &Context, _t| {
        let p = device::MouseListener::get_position(ctx);
        println!("{}: {}, {}", msg, p.x, p.y);
        Ok(())
    })
}

fn sample_keyboard_closure(msg: &'static str) -> Box<dyn Fn(&ggez::Context, Clock) -> Result<(), String>> {
    Box::new(move |_ctx: &Context, _t| {
        println!("key event ====> {}", msg);
        Ok(())
    })
}

impl<'a> State<'a> {
    fn new(ctx: &mut Context, image: tobj::SimpleObject<'a>) -> GameResult<State<'a>> {
        let font = graphics::Font::new(ctx, "/azuki.ttf")?;

//        let mut raw_text = graphics::Text::new("Hello");
//        raw_text.set_font(font, graphics::Scale {x: 48.0, y: 48.0});

        let text = tobj::SimpleText::new(
            MovableText::new(
                "Hello".to_owned(),
                trojan::numeric::Point2f { x: 0.0, y: 0.0 },
                trojan::numeric::Vector2f { x: 1.0, y: 1.0 },
                0.0,
                0,
                Box::new(move |p: & dyn MovableObject, t: Clock| {
                    trojan::numeric::Point2f{x: p.get_position().x + 1.0, y: p.get_position().y}
                }),
                trojan::graphics::object::FontInformation::new(font, ggraphics::Scale{ x: 30.0, y: 30.0 }),
                0),
            vec![]);
        
        let mut s = State {
            frames: 0,
            text: text,
            mouse: device::MouseListener::new(),
            key: device::KeyboardListener::new_masked(
                vec![device::KeyInputDevice::GenericKeyboard],
                vec![device::VirtualKey::Action1, device::VirtualKey::Action2]),
            image: image
        };

        /*
        * indirect closure inserting
        */
        let p = sample_mouse_closure("sample_closure!!");
        s.mouse
            .register_event_handler(
                MouseButton::Left,
                device::MouseButtonEvent::Clicked,
                p
            );

        /*
         * direct closure inserting with closure returing func
         */
        s.mouse
            .register_event_handler(
                MouseButton::Left,
                device::MouseButtonEvent::Pressed,
                sample_mouse_closure("Left button is Pressed!!"));

        /*
         * direct closure inserting with lambda
         */
        s.mouse
            .register_event_handler(
                MouseButton::Left,
                device::MouseButtonEvent::Dragged,
                Box::new(move |_ctx: &Context, _t| { println!("Dragging!!"); Ok(()) }));

        s.key
            .register_event_handler(
                device::VirtualKey::Action1,
                device::KeyboardEvent::FirstPressed,
                sample_keyboard_closure("Pressed!!")
            );

        s.key
            .register_event_handler(
                device::VirtualKey::Action2,
                device::KeyboardEvent::FirstPressed,
                Box::new(move |_ctx: &Context, _t| { Ok(()) }));

        Ok(s)
    }
}

impl<'a> ggez::event::EventHandler for State<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.mouse.update(ctx, 0);
        self.key.update(ctx, 0);
        let p = self.mouse.get_last_clicked(MouseButton::Left);
        println!("{}, {}", p.x, p.y);
        self.image.move_with_func(0);
        self.text.move_with_func(0);
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 0.0].into());

        let offset = self.frames as f32 / 10.0;
        let dest_point = nalgebra::Point2::new(offset, offset);
        self.text.draw(ctx)?;
        self.image.set_alpha(0.1);
        self.image.draw(ctx)?;
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
    let image = tobj::SimpleObject::new(
        MovableUniTexture::new(&textures[0],
                               trojan::numeric::Point2f { x: 0.0, y: 0.0 },
                               trojan::numeric::Vector2f { x: 1.0, y: 1.0 },
                               0.0,
                               0,
                               Box::new(move |p: & dyn MovableObject, t: Clock| {
                                   trojan::numeric::Point2f{x: p.get_position().x + 1.0, y: p.get_position().y}
                               }),
                               0),
        vec![]
    );
    let state = &mut State::new(ctx, image).unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
