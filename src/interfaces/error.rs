use crate::{context::JsContext, object::JsObject, string::JsString, value::JsValue};

/// The JavaScriptCore [`Error`] interface.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Error<'ctx> {
    obj: JsObject<'ctx>,
}

macro_rules! property {
    ($name:ident) => {
        #[inline]
        pub fn $name(&self, ctx: JsContext<'ctx>) -> JsValue<'ctx> {
            self.obj
                .get_property(ctx, &JsString::new_from_str(stringify!($name)))
        }
    };
}

impl<'ctx> Error<'ctx> {
    #[inline(always)]
    pub const fn as_value(&self) -> JsValue<'ctx> {
        self.obj.as_value()
    }

    property!(name);
    property!(cause);
    property!(message);
    property!(stack);
}
