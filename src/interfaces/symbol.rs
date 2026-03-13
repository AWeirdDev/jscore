use crate::{context::JsContext, string::JsString, value::JsValue};

/// Represents a JavaScript symbol.
pub struct Symbol;

impl Symbol {
    /// Create a symbol without description.
    #[inline]
    pub fn new<'ctx>(ctx: JsContext<'ctx>) -> JsValue<'ctx> {
        JsValue::new_symbol(ctx, None)
    }

    /// Create a symbol with description.
    #[inline]
    pub fn new_with_description<'ctx>(
        ctx: JsContext<'ctx>,
        description: &JsString,
    ) -> JsValue<'ctx> {
        JsValue::new_symbol(ctx, Some(description))
    }
}
