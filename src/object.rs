use std::{ffi::c_void, marker::PhantomData, ptr::null_mut};

use crate::{bindings::*, context::JsContext};

pub struct JsObject<'ctx> {
    _phantom: PhantomData<&'ctx ()>,
    pub(crate) rf: JsObjectRef,
}

impl<'ctx> JsObject<'ctx> {
    /// Creates a JavaScript object.
    pub fn new(ctx: &JsContext<'ctx>, data: Option<*mut c_void>) -> Self {
        Self {
            _phantom: PhantomData,
            rf: unsafe { js_object_make(ctx.rf, null_mut(), data.unwrap_or_else(|| null_mut())) },
        }
    }
}
