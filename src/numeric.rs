extern crate nalgebra;

pub type Point2i = nalgebra::geometry::Point2<i32>;
pub type Point2i64 = nalgebra::geometry::Point2<i64>;

pub type Point2u = nalgebra::geometry::Point2<u32>;
pub type Point2u64 = nalgebra::geometry::Point2<u64>;

pub type Point2f = nalgebra::geometry::Point2<f32>;
pub type Point2f64 = nalgebra::geometry::Point2<f64>;

pub type Point3i = nalgebra::geometry::Point3<i32>;
pub type Point3i64 = nalgebra::geometry::Point3<i64>;

pub type Point3u = nalgebra::geometry::Point3<u32>;
pub type Point3u64 = nalgebra::geometry::Point3<u64>;

pub type Point3f = nalgebra::geometry::Point3<f32>;
pub type Point3f64 = nalgebra::geometry::Point3<f64>;

pub type Vector2i = nalgebra::base::Vector2<i32>;
pub type Vector2i64 = nalgebra::base::Vector2<i64>;

pub type Vector2u = nalgebra::base::Vector2<u32>;
pub type Vector2u64 = nalgebra::base::Vector2<u64>;

pub type Vector2f = nalgebra::base::Vector2<f32>;
pub type Vector2f64 = nalgebra::base::Vector2<f64>;

pub type Vector3i = nalgebra::base::Vector3<i32>;
pub type Vector3i64 = nalgebra::base::Vector3<i64>;

pub type Vector3u = nalgebra::base::Vector3<u32>;
pub type Vector3u64 = nalgebra::base::Vector3<u64>;

pub type Vector3f = nalgebra::base::Vector3<f32>;
pub type Vector3f64 = nalgebra::base::Vector3<f64>;

pub type Rect = ggez::graphics::Rect;

#[macro_export]
macro_rules! zero_distance {
    ($p: expr) => {
        ($p.x.powf(2.0) + $p.y.powf(2.0)).sqrt()
    };
}

#[macro_export]
macro_rules! distance {
    ($v1: expr, $v2: expr) => {
        (($v1.x - $v2.x).powf(2.0) + ($v1.y - $v2.y).powf(2.0)).sqrt()
    };
}
