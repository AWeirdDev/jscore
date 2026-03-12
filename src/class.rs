use jscore_sys::*;

/// A JavaScript class.
///
/// Released when dropped.
pub struct JsClass {
    pub(crate) rf: Option<JsClassRef>,
}

impl JsClass {
    /// Create a class of no attributes.
    pub fn new_empty() -> Self {
        let definition = unsafe { K_JS_CLASS_DEFINITION_EMPTY };

        Self {
            rf: Some(unsafe { js_class_create(&raw const definition) }),
        }
    }

    /// Releases a JavaScript class.
    ///
    /// Requires ownership.
    #[inline]
    pub fn release(mut self) -> bool {
        if let Some(rf) = self.rf.take() {
            unsafe { js_class_release(rf) };
            true
        } else {
            false
        }
    }

    /// Releases a JavaScript class without ownership checks.
    pub unsafe fn release_unchecked(&self) -> bool {
        if let Some(rf) = self.rf {
            unsafe { js_class_release(rf) };
            true
        } else {
            false
        }
    }
}
