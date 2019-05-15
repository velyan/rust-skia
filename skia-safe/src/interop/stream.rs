/// SkStream and relatives.
/// This implementation covers the minimal subset to interface with Rust or Rust streams
/// The policy is to avoid exporting Skia streams and try to cover every use case by
/// using Rust native Streams.

use crate::prelude::*;
use crate::core::Data;
use skia_bindings::{
    SkDynamicMemoryWStream,
    C_SkDynamicMemoryWStream_detachAsData,
    C_SkDynamicMemoryWStream_destruct,
    C_SkDynamicMemoryWStream_Construct,
    SkStreamAsset,
    C_SkStream_MakeFromFile,
    C_SkStream_destruct,
    C_SkStreamAsset_destruct,
    SkStream,
    C_SkStreamAsset_getLength,
    SkMemoryStream,
    C_SkMemoryStream_destruct,
    C_SkMemoryStream_ConstructFromData,
};

use std::ffi::CStr;
use std::pin::Pin;
use std::ptr;


pub type DynamicMemoryWStream = Handle<SkDynamicMemoryWStream>;
// pub type StreamAsset = Handle<SkStreamAsset>;
pub type Stream = Handle<SkStream>;
pub type MemoryStream = Handle<SkMemoryStream>;

impl NativeDrop for SkDynamicMemoryWStream {
    fn drop(&mut self) {
        unsafe {
            C_SkDynamicMemoryWStream_destruct(self);
        }
    }
}

// impl NativeDrop for SkStream {
//     fn drop(&mut self) {
//         unsafe {
//            C_SkStream_destruct(self);
//         }
//     }
// }

// impl NativeDrop for SkStreamAsset {
//     fn drop(&mut self) {
//         unsafe {
//            C_SkStreamAsset_destruct(self);
//         }
//     }
// }

impl NativeDrop for SkMemoryStream {
    fn drop(&mut self) {
        unsafe {
           C_SkMemoryStream_destruct(self);
        }
    }
}

impl Handle<SkDynamicMemoryWStream> {
    pub fn new() -> Self {
        Self::construct_c(C_SkDynamicMemoryWStream_Construct)
    }

    pub fn detach_as_data(&mut self) -> Data {
        Data::from_ptr(unsafe {
            C_SkDynamicMemoryWStream_detachAsData(self.native_mut())
        }).unwrap()
    }
}

impl Handle<SkMemoryStream> {
    pub fn from_data(data: &Data) -> Self {
        Self::construct(|stream| unsafe { C_SkMemoryStream_ConstructFromData(stream, data.shared_native()) })
    }
}





// pub struct StreamAsset {
//     pub asset: Pin<Box<SkStreamAsset>>,
// }

// impl StreamAsset {
//     /// Creates a new stream asset.
//     pub fn new(path: &str) -> StreamAsset {
//         let asset = Box::pin(unsafe{
//             let c_str = CStr::from_ptr(path.as_ptr() as *const i8);
//             let stream = C_SkStream_MakeFromFile(c_str.as_ptr());
//             let derefed_val: SkStreamAsset = *stream;
//             derefed_val
//         });
//         StreamAsset {
//             asset,
//         }
//     }

//     pub fn length(& self) -> usize {
//         unsafe {
//             C_SkStreamAsset_getLength(self.asset.native_mut())
//         }
//     }
// }
// impl Handle<SkStreamAsset> {
//      pub fn new(path: &str) -> Self {
//         Self::from_native(
//             unsafe {
//             let c_str = CStr::from_ptr(path.as_ptr() as *const i8);
//             C_SkStream_MakeFromFile(c_str.as_ptr()) as SkStreamAsset
//         })
        
        
//         // Self::from_native(unsafe { C_SkStream_MakeFromFile(cstr.as_ptr()) })
//         // Stream::from_ptr(unsafe {
//         //     C_SkStream_MakeFromFile(cstr.as_ptr())
//         // }).unwrap()
//     }


// }
    // pub fn new(path: String) -> Self {
    //     let c_str = unsafe { CStr::from_ptr(path.as_ptr() as *const i8) };
    //     let ptr = unsafe {
    //             C_SkStream_MakeFromFile(c_str.as_ptr())
    //         };
    //     Self::construct(ptr)
    // }
// }

#[test]
fn detaching_empty_dynamic_memory_w_stream_leads_to_non_null_data() {
    let mut stream = DynamicMemoryWStream::new();
    let data = stream.detach_as_data();
    assert_eq!(0, data.size())
}
