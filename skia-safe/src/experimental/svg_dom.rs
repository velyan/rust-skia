use std::ops::{Deref, DerefMut};
use crate::prelude::*;
use crate::core::{Size, Canvas};
use crate::interop::stream::{ MemoryStream, NativeStreamBase };

use skia_bindings::{
    SkSVGDOM,
    C_SkSVGDOM_MakeFromStream,
    C_SkSVGDOM_SetContainerSize,
    C_SkSVGDOM_ContainerSize,
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

impl RCHandle<SkSVGDOM> {

   pub fn from_stream(stream: &mut MemoryStream) -> Self {
        Self::from_ptr(unsafe {
            let native_stream = stream.native_mut().as_stream_mut();
            skia_bindings::C_SkSVGDOM_MakeFromStream(native_stream)
        }).unwrap()
    }

    pub fn set_container_size(&mut self, size: &Size) -> &mut Self {
        unsafe {
            skia_bindings::C_SkSVGDOM_SetContainerSize(
                self.native_mut(), *size.native())
        }
        self
    }

    pub fn container_size(&mut self) -> Size {
        Size::from_native(
         unsafe {
            skia_bindings::C_SkSVGDOM_ContainerSize(
                self.native_mut())
        })
    }

    pub fn render(&mut self, canvas: &mut Canvas) -> &mut Self {
        unsafe {
            skia_bindings::C_SkSVGDOM_Render(
                self.native(), canvas.native_mut())
        }
        self
    }

}