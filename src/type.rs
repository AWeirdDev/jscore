use crate::bindings;

#[derive(Debug, Clone, Copy)]
pub enum JsType {
    Undefined,
    Null,
    Boolean,
    Number,
    String,
    Object,
    Symbol,
    BigInt,
}

impl JsType {
    pub(crate) const unsafe fn from_ffi(typ: bindings::JsType) -> Self {
        use bindings::*;

        #[allow(nonstandard_style)]
        match typ {
            JsType_kJSTypeUndefined => Self::Undefined,
            JsType_kJSTypeNull => Self::Null,
            JsType_kJSTypeBoolean => Self::Boolean,
            JsType_kJSTypeNumber => Self::Number,
            JsType_kJSTypeString => Self::String,
            JsType_kJSTypeObject => Self::Object,
            JsType_kJSTypeSymbol => Self::Symbol,
            JsType_kJSTypeBigInt => Self::BigInt,
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
