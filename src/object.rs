use jscore_sys::*;
use std::{ffi::c_void, marker::PhantomData, mem, ptr::null_mut};

use crate::{class::JsClass, context::JsContext, string::JsString, value::JsValue};

#[derive(Clone, Copy)]
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
    #[inline(always)]
    pub fn new(ctx: JsContext<'ctx>, class: Option<&JsClass>, data: Option<*mut c_void>) -> Self {
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

    pub fn new_array(
        ctx: JsContext<'ctx>,
        arg_count: usize,
        args: impl Iterator<Item = &'ctx JsValue<'ctx>>,
    ) -> Result<Self, JsValue<'ctx>> {
        let args = args.map(|item| item.rf).collect::<Box<[JsValueRef]>>();

        let mut exception = JsValue::new_null(ctx);
        let rf = unsafe {
            js_object_make_array(ctx.rf, arg_count, args.as_ptr(), exception.as_mut_ptr())
        };

        if rf == null_mut() {
            Err(exception)
        } else {
            Ok(Self::from_rf(rf))
        }
    }

    #[inline(always)]
    pub(crate) fn from_rf(rf: JsObjectRef) -> JsObject<'ctx> {
        Self {
            _phantom: PhantomData,
            rf,
        }
    }

    /// Casts to a [`JsValue`], dropping some type information.
    #[inline]
    pub const fn as_value(&self) -> JsValue<'ctx> {
        unsafe { mem::transmute::<_, JsValue>(self.rf) }
    }

    #[inline]
    pub fn has_property(&self, ctx: JsContext<'ctx>, name: &JsString) -> bool {
        unsafe { js_object_has_property(ctx.rf, self.rf, name.as_ptr()) }
    }

    #[inline]
    pub fn is_constructor(&self, ctx: JsContext<'ctx>) -> bool {
        unsafe { js_object_is_constructor(ctx.rf, self.rf) }
    }

    #[inline]
    pub fn is_function(&self, ctx: JsContext<'ctx>) -> bool {
        unsafe { js_object_is_function(ctx.rf, self.rf) }
    }

    pub fn get_property(
        &self,
        ctx: JsContext<'ctx>,
        name: &JsString,
    ) -> Result<JsValue<'ctx>, JsValue<'ctx>> {
        let mut exception = JsValue::new_null(ctx);
        let res = unsafe {
            js_object_get_property(ctx.rf, self.rf, name.as_ptr(), exception.as_mut_ptr())
        };

        if res == null_mut() {
            Err(exception)
        } else {
            Ok(JsValue::from_rf(res))
        }
    }

    pub fn get_property_at_index(
        &self,
        ctx: JsContext<'ctx>,
        index: u32,
    ) -> Result<JsValue<'ctx>, JsValue<'ctx>> {
        let mut exception = JsValue::new_null(ctx);
        let res = unsafe {
            js_object_get_property_at_index(ctx.rf, self.rf, index, exception.as_mut_ptr())
        };

        if res == null_mut() {
            Err(exception)
        } else {
            Ok(JsValue::from_rf(res))
        }
    }

    /// Deletes a property from the object.
    ///
    /// # Returns
    /// `Ok(())` if the delete operation succeeds, otherwise `Err(exception)`
    /// (for example, if the property has the `kJSPropertyAttributeDontDelete` attribute set).
    pub fn delete_property(
        &self,
        ctx: JsContext<'ctx>,
        name: &JsString,
    ) -> Result<(), JsValue<'ctx>> {
        let mut exception = JsValue::new_null(ctx);
        let success = unsafe {
            js_object_delete_property(ctx.rf, self.rf, name.as_ptr(), exception.as_mut_ptr())
        };

        if success { Ok(()) } else { Err(exception) }
    }

    #[inline]
    pub fn get_prototype(&self, ctx: JsContext<'ctx>) -> JsValue<'ctx> {
        JsValue::from_rf(unsafe { js_object_get_prototype(ctx.rf, self.rf) })
    }

    #[inline]
    pub fn get_private_data(&self) -> *mut c_void {
        unsafe { js_object_get_private(self.rf) }
    }
}

impl std::fmt::Debug for JsObject<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JsObject({:p})", self.rf)
    }
}
