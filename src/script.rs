use std::{marker::PhantomData, ptr::null_mut};

use crate::{context::JsContext, object::JsObject, string::JsString, value::JsValue};
use jscore_sys::*;

/// Represents a script.
///
/// It is not present in the JavaScriptCore API, and is here for the sake of cleaner code.
pub struct Script<'ctx> {
    _phantom: PhantomData<&'ctx ()>,
    script: JsStringRef,
    this: JsObjectRef,
    source: JsStringRef,
    starting_lineno: i32,
}

impl<'ctx> Script<'ctx> {
    /// Create a new script.
    #[inline]
    pub fn new(
        script: &'ctx JsString,
        this: Option<&'ctx JsObject>,
        source: Option<&'ctx JsString>,
        starting_lineno: Option<i32>,
    ) -> Self {
        Self {
            _phantom: PhantomData,
            script: script.as_ptr(),
            this: this.map(|item| item.rf).unwrap_or(null_mut()),
            source: source.map(|item| item.as_ptr()).unwrap_or(null_mut()),
            starting_lineno: starting_lineno.unwrap_or_default(),
        }
    }

    /// Instantiate a script builder.
    #[inline]
    #[must_use]
    pub fn builder() -> ScriptBuilder<'ctx> {
        ScriptBuilder {
            script: Self {
                _phantom: PhantomData,
                script: JsString::new_empty().as_ptr(),
                this: null_mut(),
                source: null_mut(),
                starting_lineno: 0,
            },
        }
    }

    /// Evaluate the script.
    pub fn evaluate(&self, ctx: JsContext<'ctx>) -> Result<JsValue<'ctx>, JsValue<'ctx>> {
        let mut exception = JsValue::new_null(ctx);
        let res = unsafe {
            js_evaluate_script(
                ctx.rf,
                self.script,
                self.this,
                self.source,
                self.starting_lineno,
                exception.as_mut_ptr(),
            )
        };

        if res == null_mut() {
            Err(exception)
        } else {
            Ok(JsValue::from_rf(res))
        }
    }
}

#[repr(transparent)]
pub struct ScriptBuilder<'ctx> {
    script: Script<'ctx>,
}

impl<'ctx> ScriptBuilder<'ctx> {
    /// Sets the script content.
    #[inline]
    #[must_use]
    pub fn script(mut self, content: &'ctx JsString) -> Self {
        self.script.script = content.as_ptr();
        self
    }

    /// Sets the `this` object.
    #[inline]
    #[must_use]
    pub fn this(mut self, obj: &'ctx JsObject) -> Self {
        self.script.this = obj.rf;
        self
    }

    #[inline]
    #[must_use]
    pub fn source_url(mut self, source: &'ctx JsString) -> Self {
        self.script.source = source.as_ptr();
        self
    }

    #[inline]
    #[must_use]
    pub fn starting_line_number(mut self, start: i32) -> Self {
        self.script.starting_lineno = start;
        self
    }

    #[inline(always)]
    #[must_use]
    pub fn build(self) -> Script<'ctx> {
        self.script
    }
}
