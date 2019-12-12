use ggez::graphics as ggraphics;
use ggez::*;
use super::super::numeric;
use crate::core::Clock;
use super::{DrawableObject, DrawableObjectEssential};
use std::rc::Rc;

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

    /// 実際に描画が行われるエリアをRectで返す
    fn get_drawing_area(&self, ctx: &mut ggez::Context) -> ggraphics::Rect;

    /// 実際に描画が行われる幅と高さを返す
    fn get_drawing_size(&self, ctx: &mut ggez::Context) -> numeric::Vector2f;

    /// 現在のテクスチャを入れ替えるメソッド
    fn replace_texture(&mut self, _texture: Rc<ggraphics::Image>)
    {}
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
pub struct MovableUniTexture {
    drwob_essential: DrawableObjectEssential,
    texture: Rc<ggraphics::Image>,
    draw_param: ggraphics::DrawParam,
    mv_essential: MovableEssential,
    birth_time: Clock,
}

impl MovableUniTexture {
    
    ///
    /// # 関連関数 new
    /// MovableUniTextureを生成する
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
    pub fn new(texture: Rc<ggraphics::Image>,
               pos: numeric::Point2f,
               scale: numeric::Vector2f,
               rotation: f32,
               drawing_depth: i8,
               mf: Box<dyn Fn(& dyn MovableObject, Clock) -> numeric::Point2f>,
               now: Clock
    ) -> MovableUniTexture {
        let mut param = ggraphics::DrawParam::new();
        param.dest = pos.into();
        param.scale = scale.into();
        param.rotation = rotation;

        // visibleはデフォルトでtrueに設定
        MovableUniTexture {
            drwob_essential: DrawableObjectEssential::new(true, drawing_depth),
            texture: texture,
            draw_param: param,
            mv_essential: MovableEssential::new(mf, now, pos),
            birth_time: now
        }
    }
}

impl DrawableObject for MovableUniTexture {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.drwob_essential.visible {
            ggraphics::draw(ctx, &*self.texture, self.draw_param)
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
        self.draw_param.dest = pos.into();
    }

    #[inline(always)]
    fn get_position(&self) -> numeric::Point2f {
        self.draw_param.dest.into()
    }

    #[inline(always)]
    fn move_diff(&mut self, offset: numeric::Vector2f) {
        self.draw_param.dest.x += offset.x;
        self.draw_param.dest.y += offset.y;
    }
}

impl TextureObject for MovableUniTexture {
    #[inline(always)]
    fn set_scale(&mut self, scale: numeric::Vector2f) {
        self.draw_param.scale = scale.into();
    }

    #[inline(always)]
    fn get_scale(&self) -> numeric::Vector2f {
        self.draw_param.scale.into()
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
        self.draw_param.offset = offset.into();
    }

    #[inline(always)]
    fn get_transform_offset(&self) -> numeric::Point2f {
        self.draw_param.offset.into()
    }

    #[inline(always)]
    fn get_drawing_area(&self, _ctx: &mut ggez::Context) -> ggraphics::Rect {
        let point = self.get_position();
        let scale = self.get_scale();
        ggraphics::Rect::new(
            point.x, point.y,
            (self.texture.width() as f32) * scale.x, (self.texture.height() as f32) * scale.y)
    }

    #[inline(always)]
    fn get_drawing_size(&self, _ctx: &mut ggez::Context) -> numeric::Vector2f {
        let scale = self.get_scale();
        numeric::Vector2f::new(
            (self.texture.width() as f32) * scale.x,
            (self.texture.height() as f32) * scale.y)
    }

    fn replace_texture(&mut self, texture: Rc<ggraphics::Image>) {
        self.texture = texture;
    }
}

impl HasBirthTime for MovableUniTexture {
    fn get_birth_time(&self) -> Clock {
        self.birth_time
    }
}

impl MovableObject for MovableUniTexture {

    fn move_with_func(&mut self, t: Clock) {
        // クロージャにはselfと経過時間を与える
        self.set_position((self.mv_essential.move_func)(self, t - self.mv_essential.mf_set_time));
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
/// # フォントの情報を持つ構造体
///
/// ## フィールド
/// ### font
/// フォント
///
/// ### scale
/// フォントのスケール
///
pub struct FontInformation {
    font: ggraphics::Font,
    scale: ggraphics::Scale,
}

impl FontInformation {
    pub fn new(font: ggraphics::Font, scale: ggraphics::Scale) -> FontInformation {
        FontInformation {
            font: font,
            scale: scale
        }
    }
}

///
/// # Move可能で描画可能なテキスト
///
/// ## フィールド
/// ### drwob_essential
/// DrawableObjectを実装するために持つ構造体
///
/// ### text
/// 文字列の実態
///
/// ### draw_param
/// 主に、Trait TextureObjectをを実装するために持つ構造体
/// 描画位置, スケールなどの情報を保持している
///
/// ### mv_essential
/// MovableObjectを実装するために必要なフィールド
///
/// ### font_info
/// フォントの種類やスケールが保持されている
///
/// ### birth_time
/// このオブジェクトが生成された時刻
///
pub struct MovableText {
    drwob_essential: DrawableObjectEssential,
    text: graphics::Text,
    font_info: FontInformation,
    draw_param: ggraphics::DrawParam,
    mv_essential: MovableEssential,
    birth_time: Clock,
}

impl MovableText {
    // 生成関数
    pub fn new(text: String,
               pos: numeric::Point2f,
               scale: numeric::Vector2f,
               rotation: f32,
               drawing_depth: i8,
               mf: Box<dyn Fn(& dyn MovableObject, Clock) -> numeric::Point2f>,
               font_info: FontInformation,
               now: Clock) -> MovableText {

        let mut param = ggraphics::DrawParam::new();
        param.dest = pos.into();
        param.scale = scale.into();
        param.rotation = rotation;
        
        let mut ret_text = MovableText {
            drwob_essential: DrawableObjectEssential::new(true, drawing_depth),
            text: ggraphics::Text::new(text),
            font_info: font_info,
            draw_param: param,
            mv_essential: MovableEssential::new(mf, now, pos),
            birth_time: now
        };

        ret_text.apply_font_information();
        ret_text
    }

    fn apply_font_information(&mut self) {
        self.text.set_font(self.font_info.font,
                           self.font_info.scale);
    }

    pub fn get_font_scale(&self) -> ggraphics::Scale {
        self.font_info.scale
    }

    pub fn set_font_scale(&mut self, scale: ggraphics::Scale) {
        self.font_info.scale = scale;
        self.apply_font_information();
    }

    pub fn change_font(&mut self, font: ggraphics::Font) {
        self.font_info.font = font;
        self.apply_font_information();
    }
    
}

impl DrawableObject for MovableText {
    #[inline(always)]
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.drwob_essential.visible {
            // textを描画する
            ggraphics::draw(ctx, &self.text, self.draw_param)
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
        self.draw_param.dest = pos.into();
    }

    #[inline(always)]
    fn get_position(&self) -> numeric::Point2f {
        self.draw_param.dest.into()
    }

    #[inline(always)]
    fn move_diff(&mut self, offset: numeric::Vector2f) {
        self.draw_param.dest.x += offset.x;
        self.draw_param.dest.y += offset.y;
    }
}

impl TextureObject for MovableText {
    #[inline(always)]
    fn set_scale(&mut self, scale: numeric::Vector2f) {
        self.draw_param.scale = scale.into();
    }

    #[inline(always)]
    fn get_scale(&self) -> numeric::Vector2f {
        self.draw_param.scale.into()
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
        self.draw_param.offset = offset.into();
    }

    #[inline(always)]
    fn get_transform_offset(&self) -> numeric::Point2f {
        self.draw_param.offset.into()
    }

    #[inline(always)]
    fn get_drawing_area(&self, ctx: &mut ggez::Context) -> ggraphics::Rect {
        let point = self.get_position();
        let scale = self.get_scale();
        ggraphics::Rect::new(
            point.x, point.y,
            (self.text.width(ctx) as f32) * scale.x, (self.text.height(ctx) as f32) * scale.y)
    }

    #[inline(always)]
    fn get_drawing_size(&self, ctx: &mut ggez::Context) -> numeric::Vector2f {
        let scale = self.get_scale();
        numeric::Vector2f::new(
            (self.text.width(ctx) as f32) * scale.x,
            (self.text.height(ctx) as f32) * scale.y)
    }
}

impl HasBirthTime for MovableText {
    #[inline(always)]
    fn get_birth_time(&self) -> Clock {
        self.birth_time
    }
}

impl MovableObject for MovableText {

    fn move_with_func(&mut self, t: Clock) {
        // クロージャにはselfと経過時間を与える
        self.set_position((self.mv_essential.move_func)(self, t - self.mv_essential.mf_set_time));
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
/// # エフェクトを掛けるためのジェネリック構造体
/// この構造体で包まれたオブジェクトはエフェクトの効果を受ける
///
/// ## フィールド
/// ### movable_object
/// MovableObject, TextureObjectトレイトを実装していなければならない。
/// エフェクトはこのオブジェクトに対して行われる。
///
/// ### geffect_essential
/// HasGenericEffectEssentialを実装するために必要なフィールド
/// エフェクトのクロージャが含まれる
///
pub struct GenericEffectableObject<T: MovableObject + TextureObject> {
    movable_object: T,
    geffect_essential: HasGenericEffectEssential,
}

impl<T: MovableObject + TextureObject> GenericEffectableObject<T> {
    // 生成関数
    pub fn new(movable_object: T,
               effects: Vec<Box<dyn Fn(&mut dyn MovableObject, &ggez::Context, Clock) -> ()>>) -> GenericEffectableObject<T> {
        GenericEffectableObject::<T> {
            movable_object: movable_object,
            geffect_essential: HasGenericEffectEssential::new(effects)
        }
    }

    pub fn ref_wrapped_object(&mut self) -> &mut T {
        &mut self.movable_object
    }
}

impl<T: MovableObject + TextureObject> DrawableObject for GenericEffectableObject<T> {
    #[inline(always)]
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        self.movable_object.draw(ctx)
    }

    #[inline(always)]
    fn hide(&mut self) {
        self.movable_object.hide()
    }

    #[inline(always)]
    fn appear(&mut self) {
        self.movable_object.appear()
    }

    #[inline(always)]
    fn is_visible(&self) -> bool {
        self.movable_object.is_visible()
    }

    #[inline(always)]
    fn set_drawing_depth(&mut self, depth: i8) {
        self.movable_object.set_drawing_depth(depth)
    }

    #[inline(always)]
    fn get_drawing_depth(&self) -> i8 {
        self.movable_object.get_drawing_depth()
    }

    #[inline(always)]
    fn set_position(&mut self, pos: numeric::Point2f) {
        self.movable_object.set_position(pos)
    }

    #[inline(always)]
    fn get_position(&self) -> numeric::Point2f {
        self.movable_object.get_position()
    }

    #[inline(always)]
    fn move_diff(&mut self, offset: numeric::Vector2f) {
        self.movable_object.move_diff(offset);
    }
}

impl<T: MovableObject + TextureObject> TextureObject for GenericEffectableObject<T> {
    #[inline(always)]
    fn set_scale(&mut self, scale: numeric::Vector2f) {
        self.movable_object.set_scale(scale)
    }

    #[inline(always)]
    fn get_scale(&self) -> numeric::Vector2f {
        self.movable_object.get_scale()
    }

    #[inline(always)]
    fn set_rotation(&mut self, rad: f32) {
        self.movable_object.set_rotation(rad)
    }

    #[inline(always)]
    fn get_rotation(&self) -> f32 {
        self.movable_object.get_rotation()
    }

    #[inline(always)]
    fn set_crop(&mut self, crop: ggraphics::Rect) {
        self.movable_object.set_crop(crop)
    }

    #[inline(always)]
    fn get_crop(&self) -> ggraphics::Rect {
        self.movable_object.get_crop()
    }

    #[inline(always)]
    fn set_drawing_color(&mut self, color: ggraphics::Color) {
        self.movable_object.set_drawing_color(color)
    }

    #[inline(always)]
    fn get_drawing_color(&self) -> ggraphics::Color {
        self.movable_object.get_drawing_color()
    }

    #[inline(always)]
    fn set_alpha(&mut self, alpha: f32) {
        self.movable_object.set_alpha(alpha)
    }

    #[inline(always)]
    fn get_alpha(&self) -> f32 {
        self.movable_object.get_alpha()
    }

    #[inline(always)]
    fn set_transform_offset(&mut self, offset: numeric::Point2f) {
        self.movable_object.set_transform_offset(offset)
    }
    
    #[inline(always)]
    fn get_transform_offset(&self) -> numeric::Point2f {
        self.movable_object.get_transform_offset()
    }

    #[inline(always)]
    fn get_drawing_area(&self, ctx: &mut ggez::Context) -> ggraphics::Rect {
        self.movable_object.get_drawing_area(ctx)
    }

    #[inline(always)]
    fn get_drawing_size(&self, ctx: &mut ggez::Context) -> numeric::Vector2f {
        self.movable_object.get_drawing_size(ctx)
    }   
}

impl<T: MovableObject + TextureObject> HasBirthTime for GenericEffectableObject<T> {
    #[inline(always)]
    fn get_birth_time(&self) -> Clock {
        self.movable_object.get_birth_time()
    }
}

impl<T: MovableObject + TextureObject> MovableObject for GenericEffectableObject<T> {

    #[inline(always)]
    fn move_with_func(&mut self, t: Clock) {
        
        // クロージャにはselfと経過時間を与える
        self.movable_object.move_with_func(t)
    }

    // 従う関数を変更する
    fn override_move_func(&mut self,
                          move_fn: Box<dyn Fn(& dyn MovableObject, Clock) -> numeric::Point2f>,
                          now: Clock) {
        self.movable_object.override_move_func(move_fn, now)
    }
}


impl<T: MovableObject + TextureObject> HasGenericEffect for GenericEffectableObject<T> {
    // 新しくエフェクトを追加するメソッド
    fn add_effect(&mut self,
                  effect: Vec<Box<dyn Fn(&mut dyn MovableObject, &ggez::Context, Clock) -> ()>>) {
        self.geffect_essential.effects_list.extend(effect)
    }
}

impl<T: MovableObject + TextureObject> Effectable for GenericEffectableObject<T> {
    // 新しくエフェクトを追加するメソッド
    fn effect(&mut self, ctx: &ggez::Context, t: Clock) {
        for f in &self.geffect_essential.effects_list {
            (f)(&mut self.movable_object, ctx, t);
        }
    }
}

pub type SimpleObject = GenericEffectableObject<MovableUniTexture>;
pub type SimpleText = GenericEffectableObject<MovableText>;

