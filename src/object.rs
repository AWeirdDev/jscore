use std::{ffi::c_void, marker::PhantomData, ptr::null_mut};

use crate::{bindings::*, class::JsClass, context::JsContext};

pub struct JsObject<'ctx> {
    _phantom: PhantomData<&'ctx ()>,
    pub(crate) rf: JsObjectRef,
}

impl<'ctx> JsObject<'ctx> {
    /// Creates a JavaScript object.
    ///
    /// # Arguments
    /// - `ctx`: The execution context to use.
    /// - `class`: The [`JsClass`] to assign to the object. Pass `None` to use the default object class.
    /// - `data`: A `*mut c_void` to set as the object’s private data. Pass `None` to specify no private data.
    pub fn new(ctx: &JsContext<'ctx>, class: Option<&JsClass>, data: Option<*mut c_void>) -> Self {
        Self {
            _phantom: PhantomData,
            rf: unsafe {
                js_object_make(
                    ctx.rf,
                    class
                        .map(|item| item.rf.unwrap_or(null_mut()))
                        .unwrap_or(null_mut()),
                    data.unwrap_or(null_mut()),
                )
            },
        }
    }

    #[inline(always)]
    pub(crate) fn from_rf(rf: JsObjectRef) -> JsObject<'ctx> {
        Self {
            _phantom: PhantomData,
            rf,
        }
    }
}
