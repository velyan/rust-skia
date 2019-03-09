use std::ffi;
use crate::prelude::*;
use crate::skia::{
    FontStyle,
    FontStyleWeight,
    FontStyleSlant,
    Data,
    GlyphId,
    Unichar,
    FontTableTag,
    Rect
};
use skia_bindings::{
    C_SkTypeface_MakeDefault,
    SkTypeface,
    C_SkTypeface_MakeFromName,
    C_SkTypeface_MakeFromData,
    SkTypeface_SerializeBehavior,
    C_SkTypeface_serialize,
    SkRefCntBase
};

pub type TypefaceSerializeBehavior = EnumHandle<SkTypeface_SerializeBehavior>;

#[allow(non_upper_case_globals)]
impl EnumHandle<SkTypeface_SerializeBehavior> {
    pub const DoIncludeData: Self = Self(SkTypeface_SerializeBehavior::kDoIncludeData);
    pub const DontIncludeData: Self = Self(SkTypeface_SerializeBehavior::kDontIncludeData);
    pub const IncludeDataIfLocal: Self = Self(SkTypeface_SerializeBehavior::kIncludeDataIfLocal);
}

/*
// not sure if we need to export that yet.
type TypefaceEncoding = EnumHandle<SkTypeface_Encoding>;

impl EnumHandle<SkTypeface_Encoding> {
    pub const UTF8: Self = Self(SkTypeface_Encoding::kUTF8_Encoding);
    pub const UTF16: Self = Self(SkTypeface_Encoding::kUTF16_Encoding);
    pub const UTF32: Self = Self(SkTypeface_Encoding::kUTF32_Encoding);
}
*/

pub type Typeface = RCHandle<SkTypeface>;

impl NativeRefCountedBase for SkTypeface {
    type Base = SkRefCntBase;

    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base._base
    }
}

impl Default for RCHandle<SkTypeface> {
    fn default() -> Self {
        Typeface::from_ptr(unsafe { C_SkTypeface_MakeDefault() }).unwrap()
    }
}

impl RCHandle<SkTypeface> {
    pub fn font_style(&self) -> FontStyle {
        unsafe { FontStyle::from_native(self.native().fontStyle()) }
    }

    pub fn is_bold(&self) -> bool {
        // does not link
        // unsafe { self.native().isBold() }
        self.font_style().weight() >= FontStyleWeight::SemiBold
    }

    pub fn is_italic(&self) -> bool {
        // unsafe { self.native().isItalic() }
        self.font_style().slant() != FontStyleSlant::Upright
    }

    pub fn is_fixed_pitch(&self) -> bool {
        unsafe { self.native().isFixedPitch() }
    }

    pub fn equal(face_a: &Typeface, face_b: &Typeface) -> bool {
        unsafe { SkTypeface::Equal(face_a.native(), face_b.native()) }
    }

    pub fn from_name(familiy_name: &str, font_style: FontStyle) -> Option<Typeface> {
        let familiy_name = ffi::CString::new(familiy_name);
        if let Result::Ok(familiy_name) = familiy_name {
            Typeface::from_ptr(unsafe {
                C_SkTypeface_MakeFromName(familiy_name.as_ptr(), *font_style.native())
            })
        } else {
            None
        }
    }

    // from_file is unsupported, because it is unclear what the
    // encoding of the path name is. from_data can be used instead.

    pub fn from_data(data: &Data, index: usize) {
        Typeface::from_ptr(
            unsafe {
                C_SkTypeface_MakeFromData(data.shared_native(), index.try_into().unwrap())
            } );
    }

    pub fn serialize(&self, behavior: TypefaceSerializeBehavior) -> Data {
        Data::from_ptr(unsafe {
            C_SkTypeface_serialize(self.native(), behavior.into_native())
        }).unwrap()
    }

    // chars_to_glyphs is unsupported, because the documentation does not make sense to me:
    // The return value does not seem to actually count the required elements of the array.
    // Use Font's text_to_glyphs 's function instead.

    pub fn unichar_to_glyph(&self, unichar: Unichar) -> GlyphId {
        unsafe { self.native().unicharToGlyph(unichar) }
    }

    pub fn count_glyphs(&self) -> usize {
        unsafe { self.native().countGlyphs().try_into().unwrap() }
    }

    pub fn count_tables(&self) -> usize {
        unsafe { self.native().countTables().try_into().unwrap() }
    }

    pub fn table_tags(&self) -> Option<Vec<FontTableTag>> {
        let mut v: Vec<FontTableTag> = vec![0; self.count_tables()];
        (unsafe { self.native().getTableTags(v.as_mut_ptr()) } != 0)
            .if_true_some(v)
    }

    pub fn table_size(&self, tag: FontTableTag) -> Option<usize> {
        let size = unsafe { self.native().getTableSize(tag) };
        if size != 0 {
            Some(size)
        } else {
            None
        }
    }

    pub fn table_data(&self, tag: FontTableTag, data: &mut [u8]) -> usize {
        unsafe {
            self.native().getTableData(tag, 0, data.len(), data.as_mut_ptr() as _)
        }
    }

    pub fn units_per_em(&self) -> Option<i32> {
        let units = unsafe { self.native().getUnitsPerEm() };
        if units != 0 {
            Some(units)
        } else {
            None
        }
    }

    // TODO: implement this
    pub fn may_support_kerning(&self) -> bool {
        true
    }

    // note: adjustments slice length must be equal to glyph's len - 1.
    pub fn kerning_pair_adjustments(&self, glyphs: &[GlyphId], adjustments: &mut [i32]) -> bool {
        (adjustments.len() == glyphs.len() + 1)
        &&
        unsafe {
            self.native().
                getKerningPairAdjustments(
                    glyphs.as_ptr(),
                    glyphs.len().try_into().unwrap(),
                    adjustments.as_mut_ptr())
        }
    }

    pub fn bounds(&self) -> Rect {
        Rect::from_native(unsafe { self.native().getBounds() })
    }
}
