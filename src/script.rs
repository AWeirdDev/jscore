use std::{marker::PhantomData, ptr::null_mut};

use crate::{bindings::*, context::JsContext, object::JsObject, string::JsString, value::JsValue};

/// Represents a script.
///
/// It is not present in the JavaScriptCore API, and is here for the sake of cleaner code.
pub struct Script<'ctx> {
    _phantom: PhantomData<&'ctx ()>,
    script: JsStringRef,
    this: JsObjectRef,
    source: JsStringRef,
    starting_lineno: i32,
    exception: *mut JsValueRef,
}

impl<'ctx> Script<'ctx> {
    /// Create a new script.
    #[inline]
    pub fn new(
        ctx: &'ctx JsContext,
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
            exception: JsValue::new_null(ctx).as_mut_ptr(),
        }
    }

    #[inline]
    pub fn builder(ctx: &'ctx JsContext) -> ScriptBuilder<'ctx> {
        ScriptBuilder {
            script: Self {
                _phantom: PhantomData,
                script: JsString::new_empty().as_ptr(),
                this: null_mut(),
                source: null_mut(),
                starting_lineno: 0,
                exception: JsValue::new_null(ctx).as_mut_ptr(),
            },
        }
    }

    /// Evaluate the script.
    #[inline]
    pub fn evaluate(&self, ctx: &'ctx JsContext) -> Result<JsValue<'ctx>, JsValue<'ctx>> {
        let res = unsafe {
            js_evaluate_script(
                ctx.rf,
                self.script,
                self.this,
                self.source,
                self.starting_lineno,
                self.exception,
            )
        };

        if res == null_mut() {
            Err(JsValue::from_rf(self.exception.cast()))
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
    #[inline]
    pub fn script(mut self, content: &'ctx JsString) -> Self {
        self.script.script = content.as_ptr();
        self
    }

    #[inline]
    pub fn this(mut self, obj: &'ctx JsObject) -> Self {
        self.script.this = obj.rf;
        self
    }

    #[inline]
    pub fn source_url(mut self, source: &'ctx JsString) -> Self {
        self.script.source = source.as_ptr();
        self
    }

    #[inline]
    pub fn starting_line_number(mut self, start: i32) -> Self {
        self.script.starting_lineno = start;
        self
    }

    #[inline(always)]
    pub fn build(self) -> Script<'ctx> {
        self.script
    }
}
