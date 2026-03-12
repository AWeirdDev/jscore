use crate::value::JsValue;

/// The JavaScriptCore [`Error`] interface.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Error<'ctx> {
    value: JsValue<'ctx>,
}

impl<'ctx> Error<'ctx> {
    #[inline(always)]
    pub const fn as_value(&self) -> JsValue<'ctx> {
        self.value
    }
}
