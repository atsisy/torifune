use ggez::graphics as ggraphics;

use super::super::numeric;

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

pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Polygon(Polygon),
}
