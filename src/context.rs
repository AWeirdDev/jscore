use std::{marker::PhantomData, mem, ptr::null_mut};

use crate::bindings::*;

/// A [`ContextGroup`] associates JavaScript contexts with one another.
/// Contexts in the same group may share and exchange JavaScript objects.
/// Sharing and exchanging JavaScript objects between contexts in different groups produces
/// undefined behavior.
#[repr(transparent)]
#[derive(Debug)]
pub struct ContextGroup {
    pub(crate) rf: JsContextGroupRef,
}

impl ContextGroup {
    /// Creates a JavaScript context group.
    #[inline]
    pub fn new() -> Self {
        Self {
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

impl Drop for ContextGroup {
    fn drop(&mut self) {
        unsafe { js_context_group_release(self.rf) }
    }
}

/// A [`JsGlobalContext`] is a [`JsContext`].
///
/// It represents the global object reference.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
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
}

/// This holds the global object and other execution state.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct JsContext<'global> {
    _phantom: PhantomData<&'global ()>,
    pub(crate) rf: JsContextRef,
}

impl<'global> JsContext<'global> {
    /// Gets the global context of a JavaScript execution context.
    pub fn get_global_context(&self) -> &'global JsGlobalContext<'global> {
        unsafe { &*(js_context_get_global_context(self.rf) as *mut JsGlobalContext) }
    }
}
