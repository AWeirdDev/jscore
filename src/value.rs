use std::{cmp, marker::PhantomData, mem, ptr::null_mut, rc::Rc};

use crate::{context::JsContext, object::JsObject, string::JsString};
use jscore_sys::*;

macro_rules! checker_fn {
    ($name:ident, $fn:ident) => {
        #[inline]
        pub fn $name(&self, ctx: JsContext<'ctx>) -> bool {
            unsafe { $fn(ctx.rf, self.rf) }
        }
    };
}

macro_rules! to_primitive_fn {
    ($name:ident, $fn:ident, $to:ty) => {
        pub fn $name(&self, ctx: JsContext<'ctx>) -> Result<$to, JsValue<'ctx>> {
            let mut exception = JsValue::new_null(ctx);
            let res = unsafe { $fn(ctx.rf, self.rf, exception.as_mut_ptr()) };

            if res == 0 && !exception.is_null(ctx) {
                Err(exception)
            } else {
                Ok(res)
            }
        }
    };
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct JsValue<'ctx> {
    _phantom: PhantomData<&'ctx ()>,
    pub(crate) rf: JsValueRef,
}

impl<'ctx> JsValue<'ctx> {
    #[inline(always)]
    pub(crate) fn from_rf(rf: JsValueRef) -> Self {
        Self {
            _phantom: PhantomData,
            rf,
        }
    }

    /// Returns a JavaScript value’s type.
    #[inline]
    pub fn get_type(&self, ctx: JsContext<'ctx>) -> crate::r#type::JsType {
        unsafe { crate::r#type::JsType::from_ffi(js_value_get_type(ctx.rf, self.rf)) }
    }

    checker_fn!(is_null, js_value_is_null);
    checker_fn!(is_boolean, js_value_is_boolean);
    checker_fn!(is_number, js_value_is_number);
    checker_fn!(is_string, js_value_is_string);
    checker_fn!(is_symbol, js_value_is_symbol);
    checker_fn!(is_object, js_value_is_object);
    checker_fn!(is_array, js_value_is_array);
    checker_fn!(is_date, js_value_is_date);

    /// Casts the reference to a mutable pointer so data
    /// can be written into it.
    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut JsValueRef {
        &raw mut self.rf
    }

    /// Creates a JavaScript value of the undefined type.
    #[inline]
    pub fn new_undefined(ctx: JsContext<'ctx>) -> Self {
        Self::from_rf(unsafe { js_value_make_undefined(ctx.rf) })
    }

    /// Creates a JavaScript value of the null type.
    #[inline]
    pub fn new_null(ctx: JsContext<'ctx>) -> Self {
        Self::from_rf(unsafe { js_value_make_null(ctx.rf) })
    }

    /// Creates a JavaScript Boolean value.
    #[inline]
    pub fn new_boolean(ctx: JsContext<'ctx>, data: bool) -> Self {
        Self::from_rf(unsafe { js_value_make_boolean(ctx.rf, data) })
    }

    /// Creates a JavaScript value of the number type.
    #[inline]
    pub fn new_number(ctx: JsContext<'ctx>, data: f64) -> Self {
        Self::from_rf(unsafe { js_value_make_number(ctx.rf, data) })
    }

    // Creates a JavaScript value of the string type.
    #[inline]
    pub fn new_string(ctx: JsContext<'ctx>, data: &JsString) -> Self {
        Self::from_rf(unsafe { js_value_make_string(ctx.rf, data.as_ptr()) })
    }

    // Creates a JavaScript value of the symbol type.
    #[inline]
    pub fn new_symbol(ctx: JsContext<'ctx>, description: Option<&JsString>) -> Self {
        let item = description
            .map(|k| k.as_ptr())
            .unwrap_or_else(|| JsString::new_empty().as_ptr());

        Self::from_rf(unsafe { js_value_make_symbol(ctx.rf, item) })
    }

    /// Creates a JavaScript value from a JSON-formatted string.
    ///
    /// # Returns
    /// A [`JsValue`] containing the parsed value, or `None` if the input is invalid.
    pub fn new_from_json(ctx: JsContext<'ctx>, data: &JsString) -> Option<Self> {
        let result = unsafe { js_value_make_from_json_string(ctx.rf, data.as_ptr()) };

        if result == null_mut() {
            None
        } else {
            Some(Self::from_rf(result))
        }
    }

    #[inline]
    pub fn to_bool(&self, ctx: JsContext<'ctx>) -> bool {
        unsafe { js_value_to_boolean(ctx.rf, self.rf) }
    }

    pub fn to_number(&self, ctx: JsContext<'ctx>) -> Result<f64, JsValue<'ctx>> {
        let mut exception = JsValue::new_null(ctx);
        let value = unsafe { js_value_to_number(ctx.rf, self.rf, exception.as_mut_ptr()) };

        if value.is_nan() {
            Err(exception)
        } else {
            Ok(value)
        }
    }

    #[inline]
    pub fn to_number_lossy(&self, ctx: JsContext<'ctx>) -> f64 {
        let value = unsafe { js_value_to_number(ctx.rf, self.rf, null_mut()) };
        value
    }

    /// Converts a JavaScript value to a string and copies the
    /// result into a JavaScript string.
    pub fn to_string_copy(&self, ctx: JsContext<'ctx>) -> Result<JsString, JsValue<'ctx>> {
        let mut exception = JsValue::new_null(ctx);
        let value = unsafe { js_value_to_string_copy(ctx.rf, self.rf, exception.as_mut_ptr()) };

        if value == null_mut() {
            Err(exception)
        } else {
            Ok(JsString { rf: Some(value) })
        }
    }

    pub fn to_string_copy_lossy(&self, ctx: JsContext<'ctx>) -> JsString {
        JsString {
            rf: Some(unsafe { js_value_to_string_copy(ctx.rf, self.rf, null_mut()) }),
        }
    }

    pub fn to_object(&self, ctx: JsContext<'ctx>) -> Result<JsObject<'ctx>, JsValue<'ctx>> {
        let mut exception = JsValue::new_null(ctx);
        let value = unsafe { js_value_to_object(ctx.rf, self.rf, exception.as_mut_ptr()) };

        if value == null_mut() {
            Err(exception)
        } else {
            Ok(JsObject::from_rf(value))
        }
    }

    #[inline]
    pub unsafe fn to_object_unchecked(&self, ctx: JsContext<'ctx>) -> JsObject<'ctx> {
        let value = unsafe { js_value_to_object(ctx.rf, self.rf, null_mut()) };
        JsObject::from_rf(value)
    }

    /// Casts to a [`JsObject`].
    #[inline]
    pub fn as_object(&self, ctx: JsContext<'_>) -> Option<JsObject<'ctx>> {
        if self.is_object(ctx) {
            Some(unsafe { self.as_object_unchecked() })
        } else {
            None
        }
    }

    /// Casts to a [`JsObject`].
    #[inline]
    pub unsafe fn as_object_unchecked(&self) -> JsObject<'ctx> {
        unsafe { mem::transmute::<_, JsObject>(self.rf) }
    }

    to_primitive_fn!(to_i32, js_value_to_int32, i32);
    to_primitive_fn!(to_i64, js_value_to_int64, i64);
    to_primitive_fn!(to_u32, js_value_to_u_int32, u32);
    to_primitive_fn!(to_u64, js_value_to_u_int64, u64);

    pub fn compare(
        &self,
        ctx: JsContext<'ctx>,
        right: &JsValue<'ctx>,
    ) -> Result<cmp::Ordering, JsValue<'ctx>> {
        let mut exception = JsValue::new_null(ctx);
        let result = unsafe { js_value_compare(ctx.rf, self.rf, right.rf, exception.as_mut_ptr()) };

        #[allow(nonstandard_style)]
        match result {
            JsRelationCondition_kJSRelationConditionUndefined => Err(exception),
            JsRelationCondition_kJSRelationConditionEqual => Ok(cmp::Ordering::Equal),
            JsRelationCondition_kJSRelationConditionGreaterThan => Ok(cmp::Ordering::Greater),
            JsRelationCondition_kJSRelationConditionLessThan => Ok(cmp::Ordering::Less),
            _ => panic!(
                "while comparing values, got constant {}, which is not one of Undefined, Equal, GreaterThan, or LessThan",
                result
            ),
        }
    }

    /// Unprotects a JavaScript value from garbage collection.
    ///
    /// You can protect a value multiple times and must unprotect it an
    /// equal number of times before it becomes eligible for garbage collection.
    #[inline]
    pub fn unprotect(&self, ctx: JsContext<'ctx>) {
        unsafe {
            js_value_unprotect(ctx.rf, self.rf);
        }
    }

    /// Protects a JavaScript value from garbage collection.
    ///
    /// Use this method when you want to store a [`JsValue`] in a global or on the heap,
    /// where the garbage collector can’t discover your reference to it.
    /// You can protect a value multiple times and must unprotect it an equal number
    ///  of times before it becomes eligible for garbage collection.
    #[inline]
    pub fn protect(&self, ctx: JsContext<'ctx>) {
        unsafe {
            js_value_protect(ctx.rf, self.rf);
        }
    }

    /// Extends the lifetime of this value without checking.
    #[inline]
    pub unsafe fn extend_lifetime_unchecked(&self) -> JsValue<'static> {
        JsValue::from_rf(self.rf)
    }

    /// Get a reference-counted handle for this value, protecting it from
    /// being GC-collected until all usages are dropped.
    #[inline]
    pub fn protected(&self, ctx: JsContext<'ctx>) -> Rc<ProtectedJsValue<'ctx>> {
        Rc::new(ProtectedJsValue::new(ctx, self))
    }
}

impl std::fmt::Debug for JsValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JsValue({:p})", self.rf)
    }
}

impl<'ctx> From<JsObject<'ctx>> for JsValue<'ctx> {
    fn from(value: JsObject<'ctx>) -> Self {
        value.as_value()
    }
}

#[derive(Clone)]
pub struct ProtectedJsValue<'ctx> {
    ctx: JsContext<'ctx>,
    value: JsValue<'ctx>,
}

impl<'ctx> ProtectedJsValue<'ctx> {
    #[inline(always)]
    fn new(ctx: JsContext<'ctx>, value: &JsValue<'ctx>) -> Self {
        value.protect(ctx);

        unsafe {
            Self {
                ctx: ctx.extend_lifetime_unchecked(),
                value: value.extend_lifetime_unchecked(),
            }
        }
    }

    /// Gets the [`JsValue`] behind the handle.
    pub fn get(&self) -> &JsValue<'ctx> {
        &self.value
    }
}

impl Drop for ProtectedJsValue<'_> {
    fn drop(&mut self) {
        self.value.unprotect(self.ctx);
    }
}
