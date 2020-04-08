use ggez::graphics as ggraphics;

use super::super::numeric;
use super::super::{DrawableComponent, DrawableObjectEssential};

pub trait MeshShape {
    fn add_to_builder<'a>(&self, builder: &'a mut ggraphics::MeshBuilder) -> &'a mut ggraphics::MeshBuilder;
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
    fn add_to_builder<'a>(&self, builder: &'a mut ggraphics::MeshBuilder) -> &'a mut ggraphics::MeshBuilder {
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

    pub fn new(pos: numeric::Point2f, radius: f32, tolerance: f32, mode: ggraphics::DrawMode, color: ggraphics::Color) -> Self {
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
    fn add_to_builder<'a>(&self, builder: &'a mut ggraphics::MeshBuilder) -> &'a mut ggraphics::MeshBuilder {
	builder.circle(self.mode, self.position, self.radius, self.tolerance, self.color)
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

    pub fn new(pos: numeric::Point2f, radius1: f32, radius2: f32,
	       tolerance: f32, mode: ggraphics::DrawMode, color: ggraphics::Color) -> Self {
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
    fn add_to_builder<'a>(&self, builder: &'a mut ggraphics::MeshBuilder) -> &'a mut ggraphics::MeshBuilder {
	builder.ellipse(self.mode, self.position, self.radius1, self.radius2, self.tolerance, self.color)
    }
}

pub struct Polygon {
    points: Vec<numeric::Point2f>,
    mode: ggraphics::DrawMode,
    color: ggraphics::Color,
}

impl Polygon {

    pub fn new(points: Vec<numeric::Point2f>, mode: ggraphics::DrawMode, color: ggraphics::Color) -> Self {
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
    fn add_to_builder<'a>(&self, builder: &'a mut ggraphics::MeshBuilder) -> &'a mut ggraphics::MeshBuilder {
	builder.polygon(self.mode, &self.points, self.color).unwrap()
    }
}

pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Ellipse(Ellipse),
    Polygon(Polygon),
}

impl MeshShape for Shape {
    fn add_to_builder<'a>(&self, builder: &'a mut ggraphics::MeshBuilder) -> &'a mut ggraphics::MeshBuilder {
	match self {
	    Shape::Rectangle(s) => s.add_to_builder(builder),
	    Shape::Circle(c) => c.add_to_builder(builder),
	    Shape::Ellipse(e) => e.add_to_builder(builder),
	    Shape::Polygon(p) => p.add_to_builder(builder),
	}
    }
}

pub struct DrawableShape {
    shape: Shape,
    mesh: ggraphics::Mesh,
    drwob_essential: DrawableObjectEssential,
    draw_param: ggraphics::DrawParam,
}

impl DrawableShape {
    pub fn new(ctx: &mut ggez::Context, shape: Shape, depth: i8, color: ggraphics::Color) -> Self {
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

    pub fn ref_shape(&self) -> &Shape {
	&self.shape
    }

    pub fn ref_shape_mut(&mut self) -> &mut Shape {
	&mut self.shape
    }
}

impl DrawableComponent for DrawableShape {
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
