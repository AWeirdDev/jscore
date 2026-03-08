use std::{marker::PhantomData, ptr::null_mut};

use crate::bindings::*;

/// A [`ContextGroup`] associates JavaScript contexts with one another.
/// Contexts in the same group may share and exchange JavaScript objects.
/// Sharing and exchanging JavaScript objects between contexts in different groups produces
/// undefined behavior.
#[repr(transparent)]
#[derive(Debug)]
pub struct ContextGroup<'group> {
    _phantom: PhantomData<&'group ()>,
    pub(crate) rf: JsContextGroupRef,
}

impl<'group> ContextGroup<'group> {
    /// Creates a JavaScript context group.
    #[inline]
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
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
    pub fn create_global_context(&self) -> JsGlobalContext<'group> {
        JsGlobalContext {
            _phantom: PhantomData,
            rf: unsafe { js_global_context_create_in_group(self.rf, null_mut()) },
        }
    }

    /// Releases a JavaScript context group.
    #[inline]
    pub fn release(self) {
        unsafe { js_context_group_release(self.rf) }
    }
}

impl Drop for ContextGroup<'_> {
    fn drop(&mut self) {
        unsafe { js_context_group_release(self.rf) }
    }
}

/// A [`JsGlobalContext`] is a [`JsContext`].
///
/// It represents the global object reference.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct JsGlobalContext<'glob> {
    _phantom: PhantomData<&'glob ()>,
    pub(crate) rf: JsGlobalContextRef,
}

impl<'glob> JsGlobalContext<'glob> {
    /// Casts to a [`JsContext`].
    ///
    /// This operation has no cost at all, since [`JsGlobalContext`]
    /// is a [`JsContext`], as stated.
    #[inline]
    pub fn as_context(&self) -> JsContext<'glob, 'glob> {
        JsContext {
            _phantom: PhantomData,
            rf: self.rf as JsContextRef,
        }
    }

    /// Retains a global JavaScript execution context.
    #[inline]
    pub fn retain(&self) {
        unsafe { js_global_context_retain(self.rf) };
    }

    /// Releases a global JavaScript execution context.
    #[inline]
    pub fn release(self) {
        unsafe { js_global_context_release(self.rf) }
    }
}

/// This holds the global object and other execution state.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct JsContext<'global, 'ctx> {
    _phantom: PhantomData<(&'ctx (), &'global ())>,
    pub(crate) rf: JsContextRef,
}

impl<'global, 'ctx> JsContext<'global, 'ctx> {
    /// Gets the global context of a JavaScript execution context.
    pub fn get_global_context(&self) -> &'ctx JsGlobalContext<'global> {
        unsafe { &*(js_context_get_global_context(self.rf) as *mut JsGlobalContext) }
    }
}
