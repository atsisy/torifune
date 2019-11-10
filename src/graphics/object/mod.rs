use ggez::graphics as ggraphics;
use ggez::*;
use super::super::numeric;

pub trait DrawableObject {
    fn draw(&self, ctx: &mut Context) -> GameResult<()>;
    fn hide(&mut self);
    fn appear(&mut self);
    fn is_visible(&self) -> bool;
    
    fn set_drawing_depth(&mut self, depth: i8);
    fn get_drawing_depth(&self) -> i8;

    fn set_position(&mut self, pos: numeric::Point2f);
    fn get_position(&self) -> numeric::Point2f;
}

pub trait TextureObject : DrawableObject {
    fn set_scale(&mut self, scale: numeric::Vector2f);
    fn get_scale(&self) -> numeric::Vector2f;

    fn set_rotation(&mut self, rad: f32);
    fn get_rotation(&self) -> f32;

    fn set_crop(&mut self, crop: ggraphics::Rect);
    fn get_crop(&self) -> ggraphics::Rect;

    fn set_drawing_color(&mut self, color: ggraphics::Color);
    fn get_drawing_color(&self) -> ggraphics::Color;

    fn set_transform_offset(&mut self, offset: numeric::Point2f);
    fn get_transform_offset(&self) -> numeric::Point2f;
}

struct DrawableObjectEssential {
    pub visible: bool,
    pub drawing_depth: i8,
}

impl DrawableObjectEssential {

    fn new(visible: bool, depth: i8) -> DrawableObjectEssential {
        DrawableObjectEssential {
            visible: visible,
            drawing_depth: depth
        }
    }
    
}

pub struct UniTextureObject<'a> {
    drwob_essential: DrawableObjectEssential,
    texture: &'a ggraphics::Image,
    draw_param: ggraphics::DrawParam,
}

impl<'a> UniTextureObject<'a> {
    pub fn new(texture: &ggraphics::Image,
           pos: numeric::Point2f,
           scale: numeric::Vector2f,
           rotation: f32,
           drawing_depth: i8
    ) -> UniTextureObject {
        let mut param = ggraphics::DrawParam::new();
        param.dest = pos;
        param.scale = scale;
        param.rotation = rotation;
        
        UniTextureObject {
            drwob_essential: DrawableObjectEssential::new(true, drawing_depth),
            texture: texture,
            draw_param: param
        }
    }
}

impl<'a> DrawableObject for UniTextureObject<'a> {
    //
    // Ok(())する必要無いのでは？ 普通にdrawの返り値を返せば良い説
    //
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.drwob_essential.visible {
            ggraphics::draw(ctx, self.texture, self.draw_param)?;
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

    fn set_position(&mut self, pos: numeric::Point2f) {
        self.draw_param.dest = pos;
    }

    fn get_position(&self) -> numeric::Point2f {
        self.draw_param.dest
    }
}

impl<'a> TextureObject for UniTextureObject<'a> {
    fn set_scale(&mut self, scale: numeric::Vector2f) {
        self.draw_param.scale = scale;
    }
    
    fn get_scale(&self) -> numeric::Vector2f {
        self.draw_param.scale
    }

    fn set_rotation(&mut self, rad: f32) {
        self.draw_param.rotation = rad;
    }
    
    fn get_rotation(&self) -> f32 {
        self.draw_param.rotation
    }

    fn set_crop(&mut self, crop: ggraphics::Rect) {
        self.draw_param.src = crop;
    }
    
    fn get_crop(&self) -> ggraphics::Rect {
        self.draw_param.src
    }

    fn set_drawing_color(&mut self, color: ggraphics::Color) {
        self.draw_param.color = color;
    }
    
    fn get_drawing_color(&self) -> ggraphics::Color {
        self.draw_param.color
    }

    fn set_transform_offset(&mut self, offset: numeric::Point2f) {
        self.draw_param.offset = offset;
    }
    
    fn get_transform_offset(&self) -> numeric::Point2f {
        self.draw_param.offset
    }
    
}
