use std::ops::{Deref, DerefMut};
use crate::prelude::*;
use crate::core::{Size, Canvas};
use crate::interop::stream::MemoryStream;

use skia_bindings::{
    SkSVGDOM,
    C_SkSVGDOM_MakeFromStream,
    C_SkSVGDOM_SetContainerSize,
    C_SkSVGDOM_Render,
    C_SkSVGDOM_ref,
    C_SkSVGDOM_unref,
    SkRefCntBase,
    SkStream,
    SkStreamAsset,
    };


pub type SVGDom = RCHandle<SkSVGDOM>;

impl NativeRefCountedBase for SkSVGDOM {
   type Base = SkRefCntBase;

   fn ref_counted_base(&self) -> &Self::Base {
       &self._base._base
    }
}

// impl NativeRefCounted for SkSVGDOM {
//    fn _ref(&self) {
//         unsafe { C_SkSVGDOM_ref(self) }
//     }

//     fn _unref(&self) {
//         unsafe { C_SkSVGDOM_unref(self) }
//     }
// }

impl RCHandle<SkSVGDOM> {

   pub fn from_stream(stream: &mut MemoryStream) -> Self {
        Self::from_ptr(unsafe {
            let native_stream = &mut stream.native_mut()._base._base._base._base._base;
            skia_bindings::C_SkSVGDOM_MakeFromStream(native_stream)
        }).unwrap()
    }
//    pub fn from_stream(stream: Box<SkStreamAsset>) -> Option<Self> {
//         Self::from_ptr(unsafe {
//             let ptr: *mut SkStreamAsset = Box::into_raw(stream);
//             let c_stream = &mut (*ptr)._base._base._base;
//             skia_bindings::C_SkSVGDOM_MakeFromStream(c_stream)
//         })
//     }

    pub fn set_container_size(&mut self, size: &Size) -> &mut Self {
        unsafe {
            skia_bindings::C_SkSVGDOM_SetContainerSize(
                self.native_mut(), *size.native())
        }
        self
    }

    pub fn render(&mut self, canvas: &mut Canvas) -> &mut Self {
        unsafe {
            skia_bindings::C_SkSVGDOM_Render(
                self.native(), canvas.native_mut())
        }
        self
    }

}