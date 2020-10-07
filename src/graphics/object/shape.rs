use std::ops::{Deref, DerefMut};

use ggez::graphics as ggraphics;
use ggraphics::Drawable;

use crate::graphics::drawable::{DrawableComponent, DrawableObjectEssential};
use crate::numeric;

pub trait MeshShape {
    fn add_to_builder<'a>(
        &self,
        builder: &'a mut ggraphics::MeshBuilder,
    ) -> &'a mut ggraphics::MeshBuilder;
}

pub struct Rectangle {
    bounds: numeric::Rect,
    mode: ggraphics::DrawMode,
    color: ggraphics::Color,
}

impl Rectangle {
    pub fn new(bounds: numeric::Rect, mode: ggraphics::DrawMode, color: ggraphics::Color) -> Self {
        Rectangle {
            bounds: bounds,
            mode: mode,
            color: color,
        }
    }

    pub fn get_bounds(&self) -> numeric::Rect {
        self.bounds
    }

    pub fn get_mode(&self) -> ggraphics::DrawMode {
        self.mode
    }

    pub fn get_color(&self) -> ggraphics::Color {
        self.color
    }

    pub fn change_position(&mut self, pos: numeric::Point2f) {
        self.bounds.x = pos.x;
        self.bounds.y = pos.y;
    }

    pub fn set_color(&mut self, color: ggraphics::Color) {
        self.color = color;
    }

    pub fn change_mode(&mut self, mode: ggraphics::DrawMode) {
        self.mode = mode;
    }
}

impl MeshShape for Rectangle {
    fn add_to_builder<'a>(
        &self,
        builder: &'a mut ggraphics::MeshBuilder,
    ) -> &'a mut ggraphics::MeshBuilder {
        builder.rectangle(self.mode, self.bounds, self.color)
    }
}

pub struct Circle {
    position: numeric::Point2f,
    radius: f32,
    tolerance: f32,
    mode: ggraphics::DrawMode,
    color: ggraphics::Color,
}

impl Circle {
    pub fn new(
        pos: numeric::Point2f,
        radius: f32,
        tolerance: f32,
        mode: ggraphics::DrawMode,
        color: ggraphics::Color,
    ) -> Self {
        Circle {
            position: pos,
            radius: radius,
            tolerance: tolerance,
            mode: mode,
            color: color,
        }
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_mode(&self) -> ggraphics::DrawMode {
        self.mode
    }

    pub fn add_radius(&mut self, offset: f32) {
	self.radius += offset;
    }

    pub fn get_color(&self) -> ggraphics::Color {
        self.color
    }

    pub fn get_tolerance(&self) -> f32 {
        self.tolerance
    }

    pub fn get_position(&self) -> numeric::Point2f {
        self.position
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn change_position(&mut self, pos: numeric::Point2f) {
        self.position = pos;
    }

    pub fn set_color(&mut self, color: ggraphics::Color) {
        self.color = color;
    }

    pub fn change_mode(&mut self, mode: ggraphics::DrawMode) {
        self.mode = mode;
    }

    pub fn set_tolerance(&mut self, t: f32) {
        self.tolerance = t;
    }
}

impl MeshShape for Circle {
    fn add_to_builder<'a>(
        &self,
        builder: &'a mut ggraphics::MeshBuilder,
    ) -> &'a mut ggraphics::MeshBuilder {
        builder.circle(
            self.mode,
            self.position,
            self.radius,
            self.tolerance,
            self.color,
        )
    }
}

pub struct Ellipse {
    mode: ggraphics::DrawMode,
    position: numeric::Point2f,
    radius1: f32,
    radius2: f32,
    tolerance: f32,
    color: ggraphics::Color,
}

impl Ellipse {
    pub fn new(
        pos: numeric::Point2f,
        radius1: f32,
        radius2: f32,
        tolerance: f32,
        mode: ggraphics::DrawMode,
        color: ggraphics::Color,
    ) -> Self {
        Ellipse {
            position: pos,
            radius1: radius1,
            radius2: radius2,
            tolerance: tolerance,
            mode: mode,
            color: color,
        }
    }

    pub fn get_radius1(&self) -> f32 {
        self.radius1
    }

    pub fn get_radius2(&self) -> f32 {
        self.radius2
    }

    pub fn get_mode(&self) -> ggraphics::DrawMode {
        self.mode
    }

    pub fn get_color(&self) -> ggraphics::Color {
        self.color
    }

    pub fn get_tolerance(&self) -> f32 {
        self.tolerance
    }

    pub fn get_position(&self) -> numeric::Point2f {
        self.position
    }

    pub fn set_radius1(&mut self, radius: f32) {
        self.radius1 = radius;
    }

    pub fn set_radius2(&mut self, radius: f32) {
        self.radius2 = radius;
    }

    pub fn change_position(&mut self, pos: numeric::Point2f) {
        self.position = pos;
    }

    pub fn set_color(&mut self, color: ggraphics::Color) {
        self.color = color;
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        let mut color = self.get_color();
        color.a = alpha;
        self.set_color(color);
    }

    pub fn get_alpha(&self) -> f32 {
        self.get_color().a
    }

    pub fn change_mode(&mut self, mode: ggraphics::DrawMode) {
        self.mode = mode;
    }

    pub fn set_tolerance(&mut self, t: f32) {
        self.tolerance = t;
    }
}

impl MeshShape for Ellipse {
    fn add_to_builder<'a>(
        &self,
        builder: &'a mut ggraphics::MeshBuilder,
    ) -> &'a mut ggraphics::MeshBuilder {
        builder.ellipse(
            self.mode,
            self.position,
            self.radius1,
            self.radius2,
            self.tolerance,
            self.color,
        )
    }
}

pub struct Polygon {
    points: Vec<numeric::Point2f>,
    mode: ggraphics::DrawMode,
    color: ggraphics::Color,
}

impl Polygon {
    pub fn new(
        points: Vec<numeric::Point2f>,
        mode: ggraphics::DrawMode,
        color: ggraphics::Color,
    ) -> Self {
        Polygon {
            points: points,
            mode: mode,
            color: color,
        }
    }

    pub fn get_points(&self) -> &Vec<numeric::Point2f> {
        &self.points
    }

    pub fn get_mode(&self) -> ggraphics::DrawMode {
        self.mode
    }

    pub fn get_color(&self) -> ggraphics::Color {
        self.color
    }

    pub fn reset_points(&mut self, points: Vec<numeric::Point2f>) {
        self.points = points;
    }

    pub fn set_color(&mut self, color: ggraphics::Color) {
        self.color = color;
    }

    pub fn change_mode(&mut self, mode: ggraphics::DrawMode) {
        self.mode = mode;
    }
}

impl MeshShape for Polygon {
    fn add_to_builder<'a>(
        &self,
        builder: &'a mut ggraphics::MeshBuilder,
    ) -> &'a mut ggraphics::MeshBuilder {
        builder
            .polygon(self.mode, &self.points, self.color)
            .unwrap()
    }
}

pub struct RadiusRect {
    pos_rect: numeric::Rect,
    borders: [numeric::Vector2f; 4],
    mode: ggraphics::DrawMode,
    color: ggraphics::Color,
}

impl RadiusRect {
    ///
    /// # borders -> [top-left, top-right, bottom-right, bottom-left]
    ///
    pub fn new(
	pos_rect: numeric::Rect,
	borders: [numeric::Vector2f; 4],
	mode: ggraphics::DrawMode,
	color: ggraphics::Color
    ) -> Self {
	RadiusRect {
	    pos_rect: pos_rect,
	    borders: borders,
	    mode: mode,
	    color: color,
	}
    }

    pub fn get_drawing_area(&self) -> numeric::Rect {
	self.pos_rect
    }
}

impl MeshShape for RadiusRect {
    fn add_to_builder<'a>(
        &self,
        builder: &'a mut ggraphics::MeshBuilder,
    ) -> &'a mut ggraphics::MeshBuilder {
	let core_rect = numeric::Rect::new(
	    self.pos_rect.x + self.borders[0].x,
	    self.pos_rect.y + self.borders[0].y,
	    self.pos_rect.w - (self.borders[0].x + self.borders[1].x.max(self.borders[2].x)),
	    self.pos_rect.h - (self.borders[0].y + self.borders[3].y.max(self.borders[2].y)),
	);

	builder
            .rectangle(
		self.mode,
		core_rect,
		self.color
	    )
	    .rectangle(
		self.mode,
		numeric::Rect::new(
		    self.pos_rect.left(),
		    self.pos_rect.top() + self.borders[0].y,
		    self.borders[0].x.max(self.borders[3].x),
		    self.pos_rect.h - self.borders[0].y - self.borders[3].y,
		),
		self.color
	    )
	    .rectangle(
		self.mode,
		numeric::Rect::new(
		    self.pos_rect.left() + self.borders[0].x,
		    self.pos_rect.top(),
		    self.pos_rect.w - self.borders[0].x - self.borders[1].x,
		    self.borders[0].y.max(self.borders[1].y),
		),
		self.color
	    )
	    .rectangle(
		self.mode,
		numeric::Rect::new(
		    self.pos_rect.right() - self.borders[1].x.max(self.borders[2].x),
		    self.pos_rect.top() + self.borders[1].y,
		    self.borders[1].x.max(self.borders[2].x),
		    self.pos_rect.h - self.borders[1].y - self.borders[2].y,
		),
		self.color
	    )
	    .rectangle(
		self.mode,
		numeric::Rect::new(
		    self.pos_rect.left() + self.borders[3].x,
		    self.pos_rect.bottom() - self.borders[0].y.max(self.borders[3].y),
		    self.pos_rect.w - self.borders[3].x - self.borders[2].x,
		    self.borders[3].y.max(self.borders[2].y),
		),
		self.color
	    )
            .ellipse(
		self.mode,
		numeric::Point2f::new(self.pos_rect.x + self.borders[0].x, self.pos_rect.y + self.borders[0].y),
		self.borders[0].x,
		self.borders[0].y,
		0.0001,
		self.color
	    )
	    .ellipse(
		self.mode,
		numeric::Point2f::new(
		    self.pos_rect.right() - self.borders[1].x,
		    self.pos_rect.top() + self.borders[1].y
		),
		self.borders[1].x,
		self.borders[1].y,
		0.0001,
		self.color
	    )
    	    .ellipse(
		self.mode,
		numeric::Point2f::new(
		    self.pos_rect.right() - self.borders[2].x,
		    self.pos_rect.bottom() - self.borders[2].y
		),
		self.borders[2].x,
		self.borders[2].y,
		0.0001,
		self.color
	    )
	    .ellipse(
		self.mode,
		numeric::Point2f::new(
		    self.pos_rect.x + self.borders[3].x,
		    self.pos_rect.bottom() - self.borders[3].y
		),
		self.borders[3].x,
		self.borders[3].y,
		0.0001,
		self.color
	    )
    }
}


pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Ellipse(Ellipse),
    Polygon(Polygon),
}

impl MeshShape for Shape {
    fn add_to_builder<'a>(
        &self,
        builder: &'a mut ggraphics::MeshBuilder,
    ) -> &'a mut ggraphics::MeshBuilder {
        match self {
            Shape::Rectangle(s) => s.add_to_builder(builder),
            Shape::Circle(c) => c.add_to_builder(builder),
            Shape::Ellipse(e) => e.add_to_builder(builder),
            Shape::Polygon(p) => p.add_to_builder(builder),
        }
    }
}

pub struct DrawableShape<S>
where S: MeshShape {
    shape: S,
    mesh: ggraphics::Mesh,
    drwob_essential: DrawableObjectEssential,
    draw_param: ggraphics::DrawParam,
}

impl<S> DrawableShape<S>
where S: MeshShape {
    pub fn new(ctx: &mut ggez::Context, shape: S, depth: i8, color: ggraphics::Color) -> Self {
        let mut builder = ggraphics::MeshBuilder::new();
        shape.add_to_builder(&mut builder);

        let mut dparam = ggraphics::DrawParam::default();
        dparam.color = color;

        DrawableShape {
            mesh: builder.build(ctx).unwrap(),
            shape: shape,
            drwob_essential: DrawableObjectEssential::new(true, depth),
            draw_param: dparam,
        }
    }

    pub fn update_shape(&mut self, ctx: &mut ggez::Context) {
	let mut builder = ggraphics::MeshBuilder::new();
        self.shape.add_to_builder(&mut builder);
	self.mesh = builder.build(ctx).unwrap();
    }

    pub fn set_blend_mode(&mut self, mode: ggraphics::BlendMode) {
	self.mesh.set_blend_mode(Some(mode));
    }
}

impl<S> DrawableComponent for DrawableShape<S>
where S: MeshShape {
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if self.is_visible() {
            ggraphics::draw(ctx, &self.mesh, self.draw_param)?;
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

impl<S> Deref for DrawableShape<S>
where S: MeshShape {
    type Target = S;
    fn deref(&self) -> &Self::Target {
	&self.shape
    }
}

impl<S> DerefMut for DrawableShape<S>
where S: MeshShape {
    fn deref_mut(&mut self) -> &mut Self::Target {
	&mut self.shape
    }
}

pub struct FramedTextBalloon {
    inner: DrawableShape<RadiusRect>,
    outer: DrawableShape<RadiusRect>,
    drwob_essential: DrawableObjectEssential,
}

impl FramedTextBalloon {
    pub fn new(
	ctx: &mut ggez::Context,
	pos_rect: numeric::Rect,
	borders: [numeric::Vector2f; 4],
	frame_width: f32,
	inner_color: ggraphics::Color,
	outer_color: ggraphics::Color,
	depth: i8,
    ) -> Self {
	FramedTextBalloon {
	    inner: DrawableShape::new(
		ctx,
		RadiusRect::new(
		    numeric::Rect::new(
			pos_rect.x + frame_width,
			pos_rect.y + frame_width,
			pos_rect.w - (frame_width * 2.0),
			pos_rect.h - (frame_width * 2.0),
		    ),
		    borders,
		    ggraphics::DrawMode::fill(),
		    inner_color
		),
		0,
		ggraphics::WHITE,
	    ),
	    outer: DrawableShape::new(
		ctx,
		RadiusRect::new(
		    pos_rect,
		    borders,
		    ggraphics::DrawMode::fill(),
		    outer_color
		),
		0,
		ggraphics::WHITE,
	    ),
	    drwob_essential: DrawableObjectEssential::new(true, depth),
	}
    }

    pub fn get_drawing_area(&self) -> numeric::Rect {
	self.outer.get_drawing_area()
    }
}

impl DrawableComponent for FramedTextBalloon {
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
	if self.is_visible() {
	    self.outer.draw(ctx)?;
	    self.inner.draw(ctx)?;
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
