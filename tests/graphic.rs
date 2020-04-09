use ggez::graphics as ggraphics;
use ggez::input::mouse::MouseButton;
use ggez::*;
use std::env;
use std::path;
use torifune::core::Clock;
use torifune::core::Updatable;
use torifune::device;
use torifune::graphics::object as tobj;
use torifune::graphics::object::*;
use torifune::graphics::DrawableComponent;
use torifune::numeric;

struct State {
    frames: usize,
    vertical_text: VerticalText,
    mouse: device::MouseListener,
    key: device::KeyboardListener,
    image: tobj::SimpleObject,
}

fn sample_mouse_closure(
    msg: &'static str,
) -> Box<dyn Fn(&ggez::Context, Clock) -> Result<(), String>> {
    Box::new(move |ctx: &Context, _t| {
        let p = device::MouseListener::get_position(ctx);
        println!("{}: {}, {}", msg, p.x, p.y);
        Ok(())
    })
}

fn sample_keyboard_closure(
    msg: &'static str,
) -> Box<dyn Fn(&ggez::Context, Clock) -> Result<(), String>> {
    Box::new(move |_ctx: &Context, _t| {
        println!("key event ====> {}", msg);
        Ok(())
    })
}

impl State {
    fn new(ctx: &mut Context, image: tobj::SimpleObject) -> GameResult<State> {
        let font = graphics::Font::new(ctx, "/azuki.ttf")?;

        //        let mut raw_text = graphics::Text::new("Hello");
        //        raw_text.set_font(font, graphics::Scale {x: 48.0, y: 48.0});

        let mut s = State {
            frames: 0,
            vertical_text: torifune::graphics::object::VerticalText::new(
                "これはテスト".to_string(),
                numeric::Point2f::new(0.0, 0.0),
                numeric::Vector2f::new(1.0, 1.0),
                0.0,
                0,
                torifune::graphics::object::FontInformation::new(
                    font,
                    numeric::Vector2f::new(24.0, 24.0),
                    ggraphics::WHITE,
                ),
            ),
            mouse: device::MouseListener::new(),
            key: device::KeyboardListener::new_masked(
                vec![device::KeyInputDevice::GenericKeyboard],
                vec![device::VirtualKey::Action1, device::VirtualKey::Action2],
            ),
            image: image,
        };

        Ok(s)
    }

    pub fn init(&mut self) {
        /*
         * indirect closure inserting
         */
        let p = sample_mouse_closure("sample_closure!!");
        self.mouse
            .register_event_handler(MouseButton::Left, device::MouseButtonEvent::Clicked, p);

        /*
         * direct closure inserting with closure returing func
         */
        self.mouse.register_event_handler(
            MouseButton::Left,
            device::MouseButtonEvent::Pressed,
            sample_mouse_closure("Left button is Pressed!!"),
        );
        /*
         * direct closure inserting with lambda
         */
        self.mouse.register_event_handler(
            MouseButton::Left,
            device::MouseButtonEvent::Dragged,
            Box::new(|_ctx: &Context, _t| {
                println!("Dragging!!");
                Ok(())
            }),
        );

        self.key.register_event_handler(
            device::VirtualKey::Action1,
            device::KeyboardEvent::FirstPressed,
            sample_keyboard_closure("Pressed!!"),
        );

        self.key.register_event_handler(
            device::VirtualKey::Action2,
            device::KeyboardEvent::FirstPressed,
            Box::new(move |_ctx: &Context, _t| Ok(())),
        );
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.mouse.update(ctx, 0);
        self.key.update(ctx, 0);
        let p = self.vertical_text.get_texture_size(ctx);
        println!("size: {}, {}", p.x, p.y);
        self.image.move_with_func(0);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 0.0].into());

        let offset = self.frames as f32 / 10.0;
        self.image.set_alpha(0.1);
        self.image.draw(ctx)?;
        self.vertical_text.draw(ctx);
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
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("test", "akichi")
        .add_resource_path(resource_dir)
        .conf(c)
        .build()
        .unwrap();

    let textures = vec![std::rc::Rc::new(
        ggraphics::Image::new(ctx, "/ghost1.png").unwrap(),
    )];
    let image = tobj::SimpleObject::new(
        MovableUniTexture::new(
            textures[0].clone(),
            torifune::numeric::Point2f::new(0.0, 0.0),
            torifune::numeric::Vector2f::new(1.0, 1.0),
            0.0,
            0,
            Box::new(move |p: &dyn MovableObject, t: Clock| {
                torifune::numeric::Point2f::new(p.get_position().x + 1.0, p.get_position().y)
            }),
            0,
        ),
        vec![],
    );
    let state = &mut State::new(ctx, image).unwrap();
    state.init();
    event::run(ctx, event_loop, state).unwrap();
}

#[test]
pub fn vertical_text() {}
