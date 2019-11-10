use ggez::graphics as ggraphics;
use ggez::*;
use super::super::numeric;

///
/// # 基本的な描画処理を保証させるトレイト
/// 汎用的なdrawインターフェイスを提供する
///
pub trait DrawableObject {
    /// このトレイトを実装する場合、このメソッドには描画を行う処理を記述する
    fn draw(&self, ctx: &mut Context) -> GameResult<()>;

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

    /// 描画開始地点を設定する
    fn set_position(&mut self, pos: numeric::Point2f);

    /// 描画開始地点を返す
    fn get_position(&self) -> numeric::Point2f;
}

///
/// # テクスチャを利用して描画を行うために必要なインターフェイスを保証させるトレイト
/// テクスチャを利用して描画を行う場合は、このトレイトを実装し、動作を保証しなければならない
///
pub trait TextureObject : DrawableObject {
    /// テクスチャのスケールを設定する
    fn set_scale(&mut self, scale: numeric::Vector2f);

    /// テクスチャのスケールを返す
    fn get_scale(&self) -> numeric::Vector2f;

    /// テクスチャの回転状態を設定する
    fn set_rotation(&mut self, rad: f32);

    /// テクスチャの回転状態を返す
    fn get_rotation(&self) -> f32;

    /// テクスチャの切り抜きを設定する。範囲は倍率（f32）で設定する
    fn set_crop(&mut self, crop: ggraphics::Rect);

    /// テクスチャの切り抜き情報を返す
    fn get_crop(&self) -> ggraphics::Rect;

    /// テクスチャを描画する際の色情報を設定する
    fn set_drawing_color(&mut self, color: ggraphics::Color);

    /// テクスチャを描画する際の色情報を返す
    fn get_drawing_color(&self) -> ggraphics::Color;

    /// テクスチャに対するエフェクトの始点を設定する
    fn set_transform_offset(&mut self, offset: numeric::Point2f);

    /// テクスチャに対するエフェクトの始点を返す
    fn get_transform_offset(&self) -> numeric::Point2f;
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

///
/// # 一つのテクスチャを扱う描画可能オブジェクト
/// 一つのテクスチャを表示する際に利用できる
///
/// ## フィールド
/// ### drwob_essential
/// DrawableObjectを実装するために持つ構造体
///
/// ### texture
/// テクスチャ。ggez::graphics::Image型への参照。
/// 参照でテクスチャを扱うため、テクスチャのコピーは行わない
///
/// ### draw_param
/// 主に、Trait TextureObjectをを実装するために持つ構造体
/// 描画位置, スケールなどの情報を保持している
///
pub struct UniTextureObject<'a> {
    drwob_essential: DrawableObjectEssential,
    texture: &'a ggraphics::Image,
    draw_param: ggraphics::DrawParam,
}

impl<'a> UniTextureObject<'a> {
    
    ///
    /// # 関連関数 new
    /// UniTextureObjectを生成する
    ///
    /// ## 引数
    /// ### texture
    /// ggez::graphics::Image型への参照
    ///
    /// ### pos
    /// 画像の描画位置
    ///
    /// ### scale
    /// テクスチャの描画スケール
    ///
    /// ### rotation
    /// テクスチャの回転角度
    ///
    /// ### drawing_depth
    /// 描画優先度
    ///
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

        // visibleはデフォルトでtrueに設定
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
