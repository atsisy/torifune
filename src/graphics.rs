pub mod object;

use std::cmp::Ordering;

use ggez::graphics as ggraphics;
use ggez::input::mouse::MouseButton;

use super::device::*;
use super::numeric;

pub type Texture = ggraphics::Image;

pub trait DrawableComponent {
    /// このトレイトを実装する場合、このメソッドには描画を行う処理を記述する
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()>;

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

    /// キー入力時の動作
    fn virtual_key_event(&mut self, _ctx: &mut ggez::Context, _event_type: KeyboardEvent, _vkey: VirtualKey) {
	// Nothing
    }

    /// マウスイベント時の動作
    fn mouse_button_event(&mut self, _ctx: &mut ggez::Context, _event_type: MouseButtonEvent,
			  _button: MouseButton, _point: numeric::Point2f) {
	// Nothing
    }
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
#[derive(Clone)]
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

///
/// Tile状に画像を切り取って表示する
///
#[derive(Clone)]
pub struct TileBatch {
    sprite_batch: ggraphics::spritebatch::SpriteBatch,
    image_size: numeric::Vector2u,
    tile_size: numeric::Vector2u,
    tile_size_ratio_float: numeric::Vector2f,
    drwob_essential: DrawableObjectEssential,
    draw_param: ggraphics::DrawParam,
}

impl TileBatch {
    pub fn new(image: ggraphics::Image,
	       tile_size: numeric::Vector2u,
	       pos: numeric::Point2f,
	       draw_depth: i8) -> Self
    {
	let param = ggraphics::DrawParam::new().dest(pos);

	let tile_size_ratio_float = numeric::Vector2f::new(
	    tile_size.x as f32 / image.width() as f32,
	    tile_size.y as f32 / image.height() as f32
	);
	
	TileBatch {
	    image_size: numeric::Vector2u::new(image.width() as u32, image.height() as u32),
	    sprite_batch: ggraphics::spritebatch::SpriteBatch::new(image),
	    tile_size: tile_size,
	    tile_size_ratio_float: tile_size_ratio_float,
	    drwob_essential: DrawableObjectEssential::new(true, draw_depth),
	    draw_param: param,
	}
    }

    ///
    /// バッチ処理を追加するメソッド
    /// 位置指定には、比率を用いる
    ///
    pub fn add_batch_ratio_float(&mut self,
			  tile_pos: numeric::Vector2f,
			  dest_pos: numeric::Point2f,
			  scale: numeric::Vector2f,
			  color: ggraphics::Color
    ) {
	self.sprite_batch.add(
	    ggraphics::DrawParam {
                src: numeric::Rect::new(
		    tile_pos.x, tile_pos.y,
		    self.tile_size_ratio_float.x, self.tile_size_ratio_float.y),
                scale: scale.into(),
                dest: dest_pos.into(),
		color: color,
                ..Default::default()
	    });
    }

    ///
    /// バッチ処理を追加するメソッド
    /// 位置指定には、タイルポジションを用いる
    ///
    pub fn add_batch_tile_position(&mut self,
				   tile_pos: numeric::Vector2u,
				   dest_pos: numeric::Point2f,
				   scale: numeric::Vector2f,
				   color: ggraphics::Color
    ) {
	// 比率表示の位置を計算
	let ratio_pos = numeric::Vector2f::new(
	    (tile_pos.x * self.tile_size.x) as f32 / self.image_size.x as f32,
	    (tile_pos.y * self.tile_size.y) as f32 / self.image_size.y as f32);

	// 比率指定でバッチ処理を追加するメソッドを呼び出す
	self.add_batch_ratio_float(ratio_pos, dest_pos, scale, color);
    }

    ///
    /// 追加したバッチ処理をクリアするメソッド
    ///
    pub fn clear_batch(&mut self) {
	self.sprite_batch.clear()
    }

    pub fn get_tile_size(&self) -> numeric::Vector2u {
	self.tile_size
    }
}

impl DrawableComponent for TileBatch {
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
	if self.is_visible() {
	    ggraphics::draw(ctx, &self.sprite_batch, self.draw_param).unwrap();
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

impl DrawableObject for TileBatch {
    fn set_position(&mut self, pos: numeric::Point2f) {
	self.draw_param.dest = pos.into();
    }

    fn get_position(&self) -> numeric::Point2f {
	self.draw_param.dest.into()
    }

    fn move_diff(&mut self, offset: numeric::Vector2f) {
	self.draw_param.dest.x += offset.x;
        self.draw_param.dest.y += offset.y;
    }
}
