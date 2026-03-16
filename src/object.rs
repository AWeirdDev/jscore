use jscore_sys::*;
use std::{ffi::c_void, marker::PhantomData, mem, ptr::null_mut};

use crate::{class::JsClass, context::JsContext, string::JsString, value::JsValue};

macro_rules! fallible {
    ($fn:ident($ctx:expr, $($args:expr,)+) -> $target:ident) => {{
        let mut exception = JsValue::new_null($ctx);
        let rf = unsafe { $fn($ctx.rf, $($args,)+ exception.as_mut_ptr()) };

        if rf == null_mut() {
            Err(exception)
        } else {
            Ok($target::from_rf(rf))
        }
    }};
}

/// Represents a JavaScript object reference.
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
                    class.map(|item| item.rf).unwrap_or(null_mut()),
                    data.unwrap_or(null_mut()),
                )
            },
        }
    }

    pub fn new_array(
        ctx: JsContext<'ctx>,
        arg_count: usize,
        args: impl Iterator<Item = JsValue<'ctx>>,
    ) -> Result<Self, JsValue<'ctx>> {
        let args = args.map(|item| item.rf).collect::<Box<[JsValueRef]>>();
        fallible!(js_object_make_array(
            ctx,
            arg_count,
            args.as_ptr(),
        ) -> Self)
    }

    pub fn new_error(
        ctx: JsContext<'ctx>,
        arg_count: usize,
        args: impl Iterator<Item = JsValue<'ctx>>,
    ) -> Result<Self, JsValue<'ctx>> {
        let args = args.map(|item| item.rf).collect::<Box<[JsValueRef]>>();
        fallible!(js_object_make_error(
            ctx,
            arg_count,
            args.as_ptr(),
        ) -> Self)
    }

    pub fn new_reg_exp(
        ctx: JsContext<'ctx>,
        arg_count: usize,
        args: impl Iterator<Item = JsValue<'ctx>>,
    ) -> Result<Self, JsValue<'ctx>> {
        let args = args.map(|item| item.rf).collect::<Box<[JsValueRef]>>();
        fallible!(js_object_make_reg_exp(
            ctx,
            arg_count,
            args.as_ptr(),
        ) -> Self)
    }

    pub fn new_date(
        ctx: JsContext<'ctx>,
        arg_count: usize,
        args: impl Iterator<Item = JsValue<'ctx>>,
    ) -> Result<Self, JsValue<'ctx>> {
        let args = args.map(|item| item.rf).collect::<Box<[JsValueRef]>>();
        fallible!(js_object_make_date(
            ctx,
            arg_count,
            args.as_ptr(),
        ) -> Self)
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
    pub fn has_property(&self, ctx: JsContext<'ctx>, name: JsString) -> bool {
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
        name: JsString,
    ) -> Result<JsValue<'ctx>, JsValue<'ctx>> {
        fallible!(js_object_get_property(ctx, self.rf, name.as_ptr(),) -> JsValue)
    }

    pub fn get_property_at_index(
        &self,
        ctx: JsContext<'ctx>,
        index: u32,
    ) -> Result<JsValue<'ctx>, JsValue<'ctx>> {
        fallible!(js_object_get_property_at_index(ctx, self.rf, index,) -> JsValue)
    }

    /// Deletes a property from the object.
    ///
    /// # Returns
    /// `Ok(())` if the delete operation succeeds, otherwise `Err(exception)`
    /// (for example, if the property has the `kJSPropertyAttributeDontDelete` attribute set).
    pub fn delete_property(
        &self,
        ctx: JsContext<'ctx>,
        name: JsString,
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

    /// Sets a pointer to private data on an object.
    ///
    /// The default object class doesn’t allocate storage for private data.
    /// Only objects that have a non-null [`JSClass`] can store private data.
    pub fn set_private_data(&self, data: *mut c_void) -> bool {
        unsafe { js_object_set_private(self.rf, data) }
    }

    pub fn call_as_function(
        &self,
        ctx: JsContext<'ctx>,
        this: JsObject,
        arg_count: usize,
        args: impl Iterator<Item = JsValue<'ctx>>,
    ) -> Result<JsValue<'ctx>, JsValue<'ctx>> {
        let args = args.map(|item| item.rf).collect::<Box<[JsValueRef]>>();
        fallible!(
            js_object_call_as_function(
                ctx,
                self.rf,
                this.rf,
                arg_count,
                args.as_ptr(),
            ) -> JsValue
        )
    }

    pub fn call_as_constructor(
        &self,
        ctx: JsContext<'ctx>,
        arg_count: usize,
        args: impl Iterator<Item = JsValue<'ctx>>,
    ) -> Result<JsValue<'ctx>, JsValue<'ctx>> {
        let args = args.map(|item| item.rf).collect::<Box<[JsValueRef]>>();
        fallible!(
            js_object_call_as_constructor(
                ctx,
                self.rf,
                arg_count,
                args.as_ptr(),
            ) -> JsValue
        )
    }

    #[inline]
    pub unsafe fn extend_lifetime_unchecked(&self) -> JsObject<'static> {
        JsObject::from_rf(self.rf)
    }
}

impl std::fmt::Debug for JsObject<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JsObject({:p})", self.rf)
    }
}
