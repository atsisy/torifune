use ggez::*;
use std::env;
use std::path;

struct State {
    frames: usize,
    text: graphics::Text,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let font = graphics::Font::new(ctx, "/azuki.ttf")?;
        let mut text = graphics::Text::new("Hello");
        text.set_font(font, graphics::Scale {x: 48.0, y: 48.0});
        
        let s = State {
            frames: 0,
            text: text,
        };

        Ok(s)
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
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

pub fn main() {
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
