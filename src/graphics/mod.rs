pub mod object;

use ggez::graphics as ggraphics;
use super::numeric;
use std::cmp::Ordering;

pub type Texture = ggraphics::Image;

pub trait DrawableComponent {
    /// このトレイトを実装する場合、このメソッドには描画を行う処理を記述する
    fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()>;

    /// このメソッドを呼び出した後は、
    /// drawメソッドを呼び出しても何も描画されなくなることを保証しなければならない
    /// appearメソッドが呼び出されれば、この効果は無効化されなければならない
    fn hide(&mut self);

    /// このメソッドを呼び出した後は、
    /// hideメソッドを呼び出していた場合でも、drawメソッドで描画できることを保証しなければならない
    /// hideメソッドが呼び出されれば、この効果は無効化されなければならない
    fn appear(&mut self);

    /// drawが有効ならtrue, そうでない場合はfalse
    fn is_visible(&self) -> bool;

    /// 描画順序を設定する
    fn set_drawing_depth(&mut self, depth: i8);

    /// 描画順序を返す
    fn get_drawing_depth(&self) -> i8;
    
}

///
/// # 基本的な描画処理を保証させるトレイト
/// 汎用的なdrawインターフェイスを提供する
///
pub trait DrawableObject : DrawableComponent {

    /// 描画開始地点を設定する
    fn set_position(&mut self, _pos: numeric::Point2f) {
    }

    /// 描画開始地点を返す
    fn get_position(&self) -> numeric::Point2f {
        numeric::Point2f::new(0.0, 0.0)
    }

    /// offsetで指定しただけ描画位置を動かす
    fn move_diff(&mut self, _offset: numeric::Vector2f) {
    }
}

///
/// DrawableObjectを深度（Z軸）でソートするための関数
///
/// この関数でソートすると、深度が深いものが先頭に来るようにソートされる
///
pub fn drawable_object_sort_with_depth<T, U>(a: &T, b: &U) -> Ordering
where T: DrawableObject,
      U: DrawableObject {
    let (ad, bd) = (a.get_drawing_depth(), b.get_drawing_depth());
    if ad > bd {
        Ordering::Less
    } else if ad < bd {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

///
/// DrawableObjectを深度（Z軸）でソートするための関数
///
/// この関数でソートすると、深度が深いものが先頭に来るようにソートされる
///
pub fn boxed_drawable_object_sort_with_depth<T, U>(a: &Box<T>, b: &Box<U>) -> Ordering
where T: DrawableObject,
      U: DrawableObject {
    let (ad, bd) = (a.get_drawing_depth(), b.get_drawing_depth());
    if ad > bd {
        Ordering::Less
    } else if ad < bd {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

///
/// # Trait DrawableObjectを実装するために必要なフィールド群
/// Trait DrawableObjectを実装する場合に便利な構造体
///
/// ## フィールド
/// ### visible
/// bool型
///
/// ### drawing_depth
/// i8型
///
pub struct DrawableObjectEssential {
    pub visible: bool,
    pub drawing_depth: i8,
}

impl DrawableObjectEssential {
    // DrawableObjectEssentialの生成関数
    pub fn new(visible: bool, depth: i8) -> DrawableObjectEssential {
        DrawableObjectEssential {
            visible: visible,
            drawing_depth: depth
        }
    }    
}

pub struct SubScreen {
    canvas: ggraphics::Canvas,
    drwob_essential: DrawableObjectEssential,
    draw_param: ggraphics::DrawParam,
    size: numeric::Vector2f,
    back_color: ggraphics::Color,
}

impl SubScreen {
    pub fn new(ctx: &mut ggez::Context, pos: ggraphics::Rect, depth: i8, back_color: ggraphics::Color) -> SubScreen {
        let mut dparam = ggraphics::DrawParam::default();
        dparam.dest = numeric::Point2f::new(pos.x, pos.y).into();
        
        SubScreen {
            canvas: ggraphics::Canvas::new(ctx, pos.w as u16, pos.h as u16, ggez::conf::NumSamples::One).unwrap(),
            drwob_essential: DrawableObjectEssential::new(true, depth),
            draw_param: dparam,
            size: numeric::Vector2f::new(pos.w, pos.h),
            back_color: back_color,
        }
    }

    pub fn begin_drawing(&self, ctx: &mut ggez::Context) {
        ggraphics::set_canvas(ctx, Some(&self.canvas));
        ggraphics::clear(ctx, self.back_color);
        ggraphics::set_screen_coordinates(ctx, ggraphics::Rect::new(0.0, 0.0, self.size.x, self.size.y)).unwrap();
    }

    pub fn end_drawing(&self, ctx: &mut ggez::Context) {
        let window_size = ggraphics::size(ctx);
        ggraphics::set_canvas(ctx, None);
        ggraphics::set_screen_coordinates(ctx, ggraphics::Rect::new(0.0, 0.0, window_size.0, window_size.1)).unwrap();
    }
}

impl DrawableComponent for SubScreen {

    fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        ggraphics::draw(ctx, &self.canvas, self.draw_param)
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

    /// 描画順序を設定する
    fn set_drawing_depth(&mut self, depth: i8) {
        self.drwob_essential.drawing_depth = depth;
    }

    /// 描画順序を返す
    fn get_drawing_depth(&self) -> i8 {
        self.drwob_essential.drawing_depth
    }

}

impl DrawableObject for SubScreen {

    /// 描画開始地点を設定する
    fn set_position(&mut self, pos: numeric::Point2f) {
        self.draw_param.dest = pos.into();
    }

    /// 描画開始地点を返す
    fn get_position(&self) -> numeric::Point2f {
        self.draw_param.dest.into()
    }

    /// offsetで指定しただけ描画位置を動かす
    fn move_diff(&mut self, offset: numeric::Vector2f) {
        self.draw_param.dest.x += offset.x;
        self.draw_param.dest.y += offset.y;
    }
}
