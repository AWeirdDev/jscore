use std::{marker::PhantomData, mem, ptr::null_mut};

use jscore_sys::*;

/// A [`ContextGroup`] associates JavaScript contexts with one another.
/// Contexts in the same group may share and exchange JavaScript objects.
/// Sharing and exchanging JavaScript objects between contexts in different groups produces
/// undefined behavior.
///
/// Released when dropped.
#[repr(transparent)]
#[derive(Debug)]
pub struct JsContextGroup {
    _no_send: PhantomData<*mut ()>,
    pub(crate) rf: JsContextGroupRef,
}

impl JsContextGroup {
    /// Creates a JavaScript context group.
    #[inline]
    pub fn new() -> Self {
        Self {
            _no_send: PhantomData,
            rf: unsafe { js_context_group_create() },
        }
    }

    /// Retains a JavaScript context group.
    #[inline]
    pub fn retain(&self) {
        unsafe { js_context_group_retain(self.rf) };
    }

    /// Creates a global JavaScript execution context in the context group.
    #[inline]
    pub fn create_global_context<'group>(&'group self) -> JsGlobalContext<'group> {
        JsGlobalContext {
            _phantom: PhantomData,
            rf: unsafe { js_global_context_create_in_group(self.rf, null_mut()) },
        }
    }
}

impl Drop for JsContextGroup {
    fn drop(&mut self) {
        unsafe { js_context_group_release(self.rf) }
    }
}

/// A [`JsGlobalContext`] is a [`JsContext`].
///
/// It represents the global object. Released when dropped.
#[repr(transparent)]
#[derive(Debug)]
pub struct JsGlobalContext<'group> {
    _phantom: PhantomData<&'group ()>,
    pub(crate) rf: JsGlobalContextRef,
}

impl<'group> JsGlobalContext<'group> {
    /// Casts to a [`JsContext`].
    ///
    /// This operation has no cost at all, since [`JsGlobalContext`]
    /// is a [`JsContext`], as stated.
    #[inline]
    pub fn as_context(&'group self) -> JsContext<'group> {
        unsafe { mem::transmute::<_, JsContext>(self.rf) }
    }

    /// Retains a global JavaScript execution context.
    #[inline]
    pub fn retain(&self) {
        unsafe { js_global_context_retain(self.rf) };
    }

    /// Sets whether the context is inspectable in Web Inspector.
    /// Default value is `false`.
    #[inline]
    pub fn set_inspectable(&self, inspectable: bool) {
        unsafe {
            js_global_context_set_inspectable(self.rf, inspectable);
        }
    }

    /// Gets whether the context is inspectable in Web Inspector.
    #[inline]
    pub fn is_inspectable(&self) -> bool {
        unsafe { js_global_context_is_inspectable(self.rf) }
    }

    /// Releases a global JavaScript execution context.
    pub fn release(self) {
        unsafe { js_global_context_release(self.rf) };
    }
}

/// This holds the global object and other execution state.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct JsContext<'global> {
    _phantom: PhantomData<&'global ()>,
    pub(crate) rf: JsContextRef,
}

impl<'global> JsContext<'global> {
    #[inline(always)]
    pub(crate) fn from_rf(rf: JsContextRef) -> Self {
        Self {
            _phantom: PhantomData,
            rf,
        }
    }

    /// Gets the global context of a JavaScript execution context.
    pub fn get_global_context(&self) -> &'global JsGlobalContext<'global> {
        unsafe { &*(js_context_get_global_context(self.rf) as *mut JsGlobalContext) }
    }

    pub unsafe fn extend_lifetime_unchecked(&self) -> JsContext<'static> {
        JsContext::from_rf(self.rf)
    }
}
