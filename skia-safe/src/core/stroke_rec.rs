use crate::prelude::*;
use crate::{paint, scalar, Paint, Path};
use skia_bindings as sb;
use skia_bindings::SkStrokeRec;

pub use sb::SkStrokeRec_InitStyle as InitStyle;

#[test]
fn test_stroke_rec_init_style_naming() {
    let _ = InitStyle::Hairline;
}

pub use sb::SkStrokeRec_Style as Style;

#[test]
fn test_stroke_rec_style_naming() {
    let _ = Style::Hairline;
}

pub type StrokeRec = Handle<SkStrokeRec>;

impl NativeDrop for SkStrokeRec {
    fn drop(&mut self) {
        unsafe { sb::C_SkStrokeRec_destruct(self) };
    }
}

impl NativeClone for SkStrokeRec {
    fn clone(&self) -> Self {
        let mut copy = StrokeRec::new_hairline();
        unsafe { sb::C_SkStrokeRec_copy(self, copy.native_mut()) }
        *copy.native()
    }
}

impl Handle<SkStrokeRec> {
    pub fn new(init_style: InitStyle) -> Self {
        Self::from_native(unsafe { SkStrokeRec::new(init_style) })
    }

    // for convenience
    pub fn new_hairline() -> Self {
        Self::new(InitStyle::Hairline)
    }

    // for convenience
    pub fn new_fill() -> Self {
        Self::new(InitStyle::Fill)
    }

    pub fn from_paint(
        paint: &Paint,
        style: impl Into<Option<paint::Style>>,
        res_scale: impl Into<Option<scalar>>,
    ) -> Self {
        let res_scale = res_scale.into().unwrap_or(1.0);
        Self::from_native(unsafe {
            match style.into() {
                Some(style) => SkStrokeRec::new1(paint.native(), style, res_scale),
                None => SkStrokeRec::new2(paint.native(), res_scale),
            }
        })
    }

    pub fn style(&self) -> Style {
        unsafe { self.native().getStyle() }
    }

    pub fn width(&self) -> scalar {
        self.native().fWidth
    }

    pub fn miter(&self) -> scalar {
        self.native().fMiterLimit
    }

    pub fn cap(&self) -> paint::Cap {
        unsafe { sb::C_SkStrokeRec_getCap(self.native()) }
    }

    pub fn join(&self) -> paint::Join {
        unsafe { sb::C_SkStrokeRec_getJoin(self.native()) }
    }

    pub fn is_hairline_style(&self) -> bool {
        self.style() == Style::Hairline
    }

    pub fn is_fill_style(&self) -> bool {
        self.style() == Style::Fill
    }

    pub fn set_fill_style(&mut self) -> &mut Self {
        unsafe { self.native_mut().setFillStyle() }
        self
    }

    pub fn set_hairline_style(&mut self) -> &mut Self {
        unsafe { self.native_mut().setHairlineStyle() }
        self
    }

    pub fn set_stroke_style(
        &mut self,
        width: scalar,
        stroke_and_fill: impl Into<Option<bool>>,
    ) -> &mut Self {
        let stroke_and_fill = stroke_and_fill.into().unwrap_or(false);
        unsafe { self.native_mut().setStrokeStyle(width, stroke_and_fill) }
        self
    }

    pub fn set_stroke_params(
        &mut self,
        cap: paint::Cap,
        join: paint::Join,
        miter_limit: scalar,
    ) -> &mut Self {
        let native = self.native_mut();
        native.set_fCap(cap as _);
        native.set_fJoin(join as _);
        native.fMiterLimit = miter_limit;
        self
    }

    pub fn res_scale(&self) -> scalar {
        self.native().fResScale
    }

    pub fn set_res_scale(&mut self, rs: scalar) {
        debug_assert!(rs > 0.0 && rs.is_finite());
        self.native_mut().fResScale = rs;
    }

    pub fn need_to_apply(&self) -> bool {
        let style = self.style();
        style == Style::Stroke || style == Style::StrokeAndFill
    }

    pub fn apply_to_path(&self, dst: &mut Path, src: &Path) -> bool {
        unsafe { self.native().applyToPath(dst.native_mut(), src.native()) }
    }

    pub fn apply_to_path_inplace(&self, path: &mut Path) -> bool {
        unsafe { self.native().applyToPath(path.native_mut(), path.native()) }
    }

    pub fn apply_to_paint(&self, paint: &mut Paint) {
        unsafe { self.native().applyToPaint(paint.native_mut()) }
    }

    pub fn inflation_radius(&self) -> scalar {
        unsafe { self.native().getInflationRadius() }
    }

    pub fn inflation_radius_from_paint_and_style(paint: &Paint, style: paint::Style) -> scalar {
        unsafe { SkStrokeRec::GetInflationRadius(paint.native(), style) }
    }

    pub fn inflation_radius_from_params(
        join: paint::Join,
        miter_limit: scalar,
        cap: paint::Cap,
        stroke_width: scalar,
    ) -> scalar {
        unsafe { SkStrokeRec::GetInflationRadius1(join, miter_limit, cap, stroke_width) }
    }

    pub fn has_equal_effect(&self, other: &StrokeRec) -> bool {
        unsafe { sb::C_SkStrokeRec_hasEqualEffect(self.native(), other.native()) }
    }
}
