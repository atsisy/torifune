use ggez::graphics as ggraphics;

use super::numeric;

use crate::graphics::*;
    
pub struct ShadowShape {
    shadow: ggraphics::Mesh,
    draw_param: ggraphics::DrawParam,
    drwob_essential: DrawableObjectEssential,
}

impl ShadowShape {
    pub fn new(ctx: &mut ggez::Context, width: f32, bounds: numeric::Rect, color: ggraphics::Color, depth: i8) -> ShadowShape {
    	let mesh = ggraphics::MeshBuilder::new()
            .rectangle(ggraphics::DrawMode::stroke(width), bounds, color)
            .build(ctx)
            .unwrap();

	let mut dparam = ggraphics::DrawParam::default();
	dparam.dest.x = 0.0;
	dparam.dest.y = 0.0;
	dparam.color = ggraphics::WHITE;
	dparam.rotation = 0.0;

	ShadowShape {
	    shadow: mesh,
	    draw_param: dparam,
	    drwob_essential: DrawableObjectEssential::new(true, depth),
	}
    }
}

impl DrawableComponent for ShadowShape {
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
	if self.drwob_essential.visible {
	    ggraphics::draw(ctx, &self.shadow, self.draw_param)?;
	}

	Ok(())
    }

    fn hide(&mut self) {
	self.drwob_essential.visible = false;
    }

    fn appear(&mut self) {
	self.drwob_essential.visible = true;
    }

    fn is_visible(&self) -> bool {
	self.drwob_essential.visible
    }

    fn set_drawing_depth(&mut self, depth: i8) {
	self.drwob_essential.drawing_depth = depth;
    }

    fn get_drawing_depth(&self) -> i8 {
	self.drwob_essential.drawing_depth
    }
}
