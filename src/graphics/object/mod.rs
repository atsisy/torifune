use ggez::graphics as ggraphics;
use ggez::*;
use super::super::numeric;
use crate::core::Clock;

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

    /// テクスチャのalpha値を設定する
    fn set_alpha(&mut self, alpha: f32);

    /// テクスチャのalpha値を返す
    fn get_alpha(&self) -> f32;

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
    // DrawableObjectEssentialの生成関数
    fn new(visible: bool, depth: i8) -> DrawableObjectEssential {
        DrawableObjectEssential {
            visible: visible,
            drawing_depth: depth
        }
    }
    
}

///
/// # 生成された時刻を記憶していることを保証させるトレイト
///
pub trait HasBirthTime {

    // 生成された時刻を返す
    fn get_birth_time(&self) -> Clock;
}

///
/// # 任意の関数に従って座標を動かすことができることを保証するトレイト
///
pub trait MovableObject : DrawableObject + HasBirthTime {    

    // 任意の関数に従って、座標を動かす
    fn move_with_func(&mut self, t: Clock);

    // 従う関数を変更する
    fn override_move_func(&mut self,
                          move_fn: Box<dyn Fn(& dyn MovableObject, Clock) -> numeric::Point2f>,
                          now: Clock);
}

///
/// # エフェクトに対応していることを保証させるトレイト
///
pub trait Effectable {
    // エフェクト処理を行う
    fn effect(&mut self, ctx: &ggez::Context, t: Clock);
}

///
/// # クロージャによって実装されるエフェクトに対応していることを保証させるトレイト
/// Effectableを実装している必要がある
///
pub trait HasGenericEffect : Effectable {
    
    // 新しくエフェクトを追加するメソッド
    fn add_effect(&mut self,
                  effect: Vec<Box<dyn Fn(&mut dyn MovableObject, &ggez::Context, Clock) -> ()>>);
}

///
/// # Trait MovableObjectを実装するために必要なフィールド群
/// Trait MovableObjectを実装する場合に便利な構造体
///
/// ## フィールド
/// ### move_func
/// 従う関数をクロージャで表現したもの。MovableObjectを実装する場合は、これに従うように実装するべき
///
/// ### mf_set_time
/// 最後にmove_funcが変更された時の時刻
///
/// ### init_position
/// 生成された時の初期位置
///
pub struct MovableEssential {
    move_func: Box<dyn Fn(& dyn MovableObject, Clock) -> numeric::Point2f>,
    mf_set_time: Clock,
    init_position: numeric::Point2f,
}

impl MovableEssential {
    // MovableEssentialを生成する関数
    fn new(f: Box<dyn Fn(& dyn MovableObject, Clock) -> numeric::Point2f>,
           t: Clock, init_pos: numeric::Point2f) -> MovableEssential {
        MovableEssential {
            move_func: f,
            mf_set_time: t,
            init_position: init_pos
        }
    }
}

///
/// # Trait Effectableを実装するために必要なフィールド群
/// Trait Effectableを実装する場合に便利な構造体
///
///
pub struct HasGenericEffectEssential {
    effects_list: Vec<Box<dyn Fn(&mut dyn MovableObject, &ggez::Context, Clock) -> ()>>,
}

impl HasGenericEffectEssential {
    fn new(list: Vec<Box<dyn Fn(&mut dyn MovableObject, &ggez::Context, Clock) -> ()>>) -> HasGenericEffectEssential {
        HasGenericEffectEssential {
            effects_list: list
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
/// ### mv_essential
/// MovableObjectを実装するために必要なフィールド
///
/// ### birth_time
/// このオブジェクトが生成された時刻
///
pub struct MovavleUniTexture<'a> {
    drwob_essential: DrawableObjectEssential,
    texture: &'a ggraphics::Image,
    draw_param: ggraphics::DrawParam,
    mv_essential: MovableEssential,
    birth_time: Clock,
}

impl<'a> MovavleUniTexture<'a> {
    
    ///
    /// # 関連関数 new
    /// MovavleUniTextureを生成する
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
               drawing_depth: i8,
               mf: Box<dyn Fn(& dyn MovableObject, Clock) -> numeric::Point2f>,
               now: Clock
    ) -> MovavleUniTexture {
        let mut param = ggraphics::DrawParam::new();
        param.dest = pos;
        param.scale = scale;
        param.rotation = rotation;

        // visibleはデフォルトでtrueに設定
        MovavleUniTexture {
            drwob_essential: DrawableObjectEssential::new(true, drawing_depth),
            texture: texture,
            draw_param: param,
            mv_essential: MovableEssential::new(mf, now, pos),
            birth_time: now
        }
    }
}

impl<'a> DrawableObject for MovavleUniTexture<'a> {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.drwob_essential.visible {
            ggraphics::draw(ctx, self.texture, self.draw_param)
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn hide(&mut self) {
        self.drwob_essential.visible = false;
    }

    #[inline(always)]
    fn appear(&mut self) {
        self.drwob_essential.visible = true;
    }

    #[inline(always)]
    fn is_visible(&self) -> bool {
        self.drwob_essential.visible
    }

    #[inline(always)]
    fn set_drawing_depth(&mut self, depth: i8) {
        self.drwob_essential.drawing_depth = depth;
    }

    #[inline(always)]
    fn get_drawing_depth(&self) -> i8 {
        self.drwob_essential.drawing_depth
    }

    #[inline(always)]
    fn set_position(&mut self, pos: numeric::Point2f) {
        self.draw_param.dest = pos;
    }

    #[inline(always)]
    fn get_position(&self) -> numeric::Point2f {
        self.draw_param.dest
    }
}

impl<'a> TextureObject for MovavleUniTexture<'a> {
    #[inline(always)]
    fn set_scale(&mut self, scale: numeric::Vector2f) {
        self.draw_param.scale = scale;
    }

    #[inline(always)]
    fn get_scale(&self) -> numeric::Vector2f {
        self.draw_param.scale
    }

    #[inline(always)]
    fn set_rotation(&mut self, rad: f32) {
        self.draw_param.rotation = rad;
    }

    #[inline(always)]
    fn get_rotation(&self) -> f32 {
        self.draw_param.rotation
    }

    #[inline(always)]
    fn set_crop(&mut self, crop: ggraphics::Rect) {
        self.draw_param.src = crop;
    }

    #[inline(always)]
    fn get_crop(&self) -> ggraphics::Rect {
        self.draw_param.src
    }

    #[inline(always)]
    fn set_drawing_color(&mut self, color: ggraphics::Color) {
        self.draw_param.color = color;
    }

    #[inline(always)]
    fn get_drawing_color(&self) -> ggraphics::Color {
        self.draw_param.color
    }

    #[inline(always)]
    fn set_alpha(&mut self, alpha: f32) {
        self.draw_param.color.a = alpha;
    }

    #[inline(always)]
    fn get_alpha(&self) -> f32 {
        self.draw_param.color.a
    }

    #[inline(always)]
    fn set_transform_offset(&mut self, offset: numeric::Point2f) {
        self.draw_param.offset = offset;
    }

    #[inline(always)]
    fn get_transform_offset(&self) -> numeric::Point2f {
        self.draw_param.offset
    }
    
}

impl<'a> HasBirthTime for MovavleUniTexture<'a> {
    fn get_birth_time(&self) -> Clock {
        self.birth_time
    }
}

impl<'a> MovableObject for MovavleUniTexture<'a> {

    fn move_with_func(&mut self, t: Clock) {
        // クロージャにはselfと経過時間を与える
        self.set_position((self.mv_essential.move_func)(self, t - self.birth_time));
    }

    // 従う関数を変更する
    fn override_move_func(&mut self,
                          move_fn: Box<dyn Fn(& dyn MovableObject, Clock) -> numeric::Point2f>,
                          now: Clock) {
        self.mv_essential.move_func = move_fn;
        self.mv_essential.mf_set_time = now;
    }
}

///
/// # 一つのテクスチャを扱う描画可能オブジェクト
/// 一つのテクスチャを表示する際に利用できる
///
/// ## フィールド
/// ### drawable_texture
/// MovableObjectトレイトを実装したもの。これに対してエフェクトを掛ける。
///
/// ### geffect_essential
/// HasGenericEffectEssentialを実装するために必要なフィールド
///
pub struct UniTextureObject<'a> {
    drawable_texture: MovavleUniTexture<'a>,
    geffect_essential: HasGenericEffectEssential,
}

impl<'a> UniTextureObject<'a> {
    pub fn new(texture: &ggraphics::Image,
           pos: numeric::Point2f,
           scale: numeric::Vector2f,
           rotation: f32,
           drawing_depth: i8,
           mf: Box<dyn Fn(& dyn MovableObject, Clock) -> numeric::Point2f>,
           now: Clock,
           effects: Vec<Box<dyn Fn(&mut dyn MovableObject, &ggez::Context, Clock) -> ()>>) -> UniTextureObject {
        UniTextureObject {
            drawable_texture: MovavleUniTexture::new(texture, pos, scale, rotation, drawing_depth, mf, now),
            geffect_essential: HasGenericEffectEssential::new(effects)
        }
    }
}


impl<'a> DrawableObject for UniTextureObject<'a> {
    #[inline(always)]
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        self.drawable_texture.draw(ctx)
    }

    #[inline(always)]
    fn hide(&mut self) {
        self.drawable_texture.hide()
    }

    #[inline(always)]
    fn appear(&mut self) {
        self.drawable_texture.appear()
    }

    #[inline(always)]
    fn is_visible(&self) -> bool {
        self.drawable_texture.is_visible()
    }

    #[inline(always)]
    fn set_drawing_depth(&mut self, depth: i8) {
        self.drawable_texture.set_drawing_depth(depth)
    }

    #[inline(always)]
    fn get_drawing_depth(&self) -> i8 {
        self.drawable_texture.get_drawing_depth()
    }

    #[inline(always)]
    fn set_position(&mut self, pos: numeric::Point2f) {
        self.drawable_texture.set_position(pos)
    }

    #[inline(always)]
    fn get_position(&self) -> numeric::Point2f {
        self.drawable_texture.get_position()
    }
}

impl<'a> TextureObject for UniTextureObject<'a> {
    #[inline(always)]
    fn set_scale(&mut self, scale: numeric::Vector2f) {
        self.drawable_texture.set_scale(scale)
    }

    #[inline(always)]
    fn get_scale(&self) -> numeric::Vector2f {
        self.drawable_texture.get_scale()
    }

    #[inline(always)]
    fn set_rotation(&mut self, rad: f32) {
        self.drawable_texture.set_rotation(rad)
    }

    #[inline(always)]
    fn get_rotation(&self) -> f32 {
        self.drawable_texture.get_rotation()
    }

    #[inline(always)]
    fn set_crop(&mut self, crop: ggraphics::Rect) {
        self.drawable_texture.set_crop(crop)
    }

    #[inline(always)]
    fn get_crop(&self) -> ggraphics::Rect {
        self.drawable_texture.get_crop()
    }

    #[inline(always)]
    fn set_drawing_color(&mut self, color: ggraphics::Color) {
        self.drawable_texture.set_drawing_color(color)
    }

    #[inline(always)]
    fn get_drawing_color(&self) -> ggraphics::Color {
        self.drawable_texture.get_drawing_color()
    }

    #[inline(always)]
    fn set_alpha(&mut self, alpha: f32) {
        self.drawable_texture.set_alpha(alpha)
    }

    #[inline(always)]
    fn get_alpha(&self) -> f32 {
        self.drawable_texture.get_alpha()
    }

    #[inline(always)]
    fn set_transform_offset(&mut self, offset: numeric::Point2f) {
        self.drawable_texture.set_transform_offset(offset)
    }
    
    #[inline(always)]
    fn get_transform_offset(&self) -> numeric::Point2f {
        self.drawable_texture.get_transform_offset()
    }
    
}

impl<'a> HasBirthTime for UniTextureObject<'a> {
    #[inline(always)]
    fn get_birth_time(&self) -> Clock {
        self.drawable_texture.get_birth_time()
    }
}

impl<'a> MovableObject for UniTextureObject<'a> {

    #[inline(always)]
    fn move_with_func(&mut self, t: Clock) {
        
        // クロージャにはselfと経過時間を与える
        self.drawable_texture.move_with_func(t)
    }

    // 従う関数を変更する
    fn override_move_func(&mut self,
                          move_fn: Box<dyn Fn(& dyn MovableObject, Clock) -> numeric::Point2f>,
                          now: Clock) {
        self.drawable_texture.override_move_func(move_fn, now)
    }
}

impl<'a> HasGenericEffect for UniTextureObject<'a> {
    // 新しくエフェクトを追加するメソッド
    fn add_effect(&mut self,
                  effect: Vec<Box<dyn Fn(&mut dyn MovableObject, &ggez::Context, Clock) -> ()>>) {
        self.geffect_essential.effects_list.extend(effect)
    }
}

impl<'a> Effectable for UniTextureObject<'a> {
    // 新しくエフェクトを追加するメソッド
    fn effect(&mut self, ctx: &ggez::Context, t: Clock) {
        for f in &self.geffect_essential.effects_list {
            (f)(&mut self.drawable_texture, ctx, t);
        }
    }
}
