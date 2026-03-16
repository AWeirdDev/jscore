use std::{
    cell::OnceCell,
    ffi::{CString, c_char, c_int, c_uint},
    marker::PhantomData,
    mem,
    ptr::null_mut,
    slice,
    thread::LocalKey,
};

use bitflags::bitflags;
use jscore_sys::*;

use crate::{context::JsContext, object::JsObject, string::JsString, value::JsValue};

/// A JavaScript class.
pub struct JsClass {
    pub(crate) rf: JsClassRef,
}

impl JsClass {
    #[inline]
    pub fn new(definition: &ReusableJsClassDefinition) -> Self {
        Self {
            rf: unsafe { js_class_create(definition.as_ptr()) },
        }
    }

    /// Create a class with an empty declaration.
    #[inline]
    pub fn new_empty() -> Self {
        let definition = unsafe { K_JS_CLASS_DEFINITION_EMPTY };

        Self {
            rf: unsafe { js_class_create(&raw const definition) },
        }
    }

    /// Converts to a pointer.
    #[inline(always)]
    pub const fn as_ptr(&self) -> JsClassRef {
        self.rf
    }

    /// Releases a JavaScript class.
    ///
    /// Requires ownership.
    #[inline(always)]
    pub unsafe fn release(self) {
        unsafe {
            self.release_unchecked();
        };
    }

    /// Releases a JavaScript class without ownership checks.
    #[inline(always)]
    pub unsafe fn release_unchecked(&self) {
        unsafe { js_class_release(self.rf) };
    }
}

bitflags! {
    /// A set of JavaScript class attributes (flags).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct JsClassAttributes: c_uint {
        const None = kJSClassAttributeNone;
        const NoAutomaticPrototype = kJSClassAttributeNoAutomaticPrototype;
    }
}

bitflags! {
    /// A set of JavaScript property attributes (flags).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct JsPropertyAttributes: c_uint {
        const None = kJSPropertyAttributeNone;
        const DontDelete = kJSPropertyAttributeDontDelete;
        const DontEnum = kJSPropertyAttributeDontEnum;
        const ReadOnly = kJSPropertyAttributeReadOnly;
    }
}

pub type ObjectPropertyGetCallback<'ctx> = Box<
    dyn Fn(JsContext<'ctx>, JsObject<'ctx>, JsString) -> Result<JsValue<'ctx>, JsValue<'ctx>>
        + 'static,
>;
pub type ObjectPropertySetCallback<'ctx> = Box<
    dyn Fn(
            JsContext<'ctx>,
            JsObject<'ctx>,
            JsString,
            Option<JsValue>,
        ) -> Result<bool, JsValue<'ctx>>
        + 'static,
>;
pub type ObjectCallAsFunctionCallback<'ctx> = Box<
    dyn Fn(
            JsContext<'ctx>,
            JsObject<'ctx>,
            JsObject<'ctx>,
            usize,
            &[JsObject<'ctx>],
        ) -> Result<JsValue<'ctx>, JsValue<'ctx>>
        + 'static,
>;

/// A statically declared value property.
#[allow(nonstandard_style)]
pub struct JsStaticValueBuilder {
    /// A null-terminated UTF-8 string that contains the property’s name.
    name: *const c_char,

    /// A callback to invoke when getting the property’s value.
    getProperty: JsObjectGetPropertyCallback,

    /// A callback to invoke when setting the property’s value.
    setProperty: JsObjectSetPropertyCallback,

    /// A set of property attributes to give to the property.
    attributes: jscore_sys::JsPropertyAttributes,

    jscore__dealloc_get: Option<LocalKey<OnceCell<ObjectPropertyGetCallback<'static>>>>,
    jscore__dealloc_set: Option<LocalKey<OnceCell<ObjectPropertySetCallback<'static>>>>,
}

impl JsStaticValueBuilder {
    /// Creates a new JavaScript static value (a statically declared value property).
    #[inline]
    pub fn new<K: AsRef<str>>(name: K) -> Self {
        let cs = CString::new(name.as_ref()).expect("found null byte in static value name");

        Self {
            name: cs.into_raw(),
            getProperty: None,
            setProperty: None,
            attributes: kJSPropertyAttributeNone,
            jscore__dealloc_get: None,
            jscore__dealloc_set: None,
        }
    }

    pub fn with_get<'ctx>(mut self, callback: ObjectPropertyGetCallback<'ctx>) -> Self {
        thread_local! {
            static CALLBACK: OnceCell<ObjectPropertyGetCallback<'static>> = OnceCell::new();
        };
        unsafe extern "C" fn wrapped_callback(
            ctx: JsContextRef,
            object: JsObjectRef,
            property_name: JsStringRef,
            exception: *mut JsValueRef,
        ) -> JsValueRef {
            CALLBACK.with(|inner| {
                let func = inner.get().unwrap();
                let res = func(
                    JsContext::from_rf(ctx),
                    JsObject::from_rf(object),
                    JsString::from_rf(property_name),
                );

                match res {
                    Ok(o) => o.rf,
                    Err(e) => {
                        // write the exception
                        unsafe {
                            *exception = e.rf;
                        }
                        null_mut()
                    }
                }
            })
        }

        // essentially, as long as 'ctx (the context) is still alive,
        // it can call this function; when it dies, this function dies
        let callback = unsafe { mem::transmute::<_, ObjectPropertyGetCallback<'static>>(callback) };
        CALLBACK.with(|inner| unsafe { inner.set(callback).unwrap_unchecked() });

        self.getProperty = Some(wrapped_callback);
        self.jscore__dealloc_get.replace(CALLBACK);

        self
    }

    pub fn with_set<'ctx>(mut self, callback: ObjectPropertySetCallback<'ctx>) -> Self {
        thread_local! {
            static CALLBACK: OnceCell<ObjectPropertySetCallback<'static>> = OnceCell::new();
        };
        unsafe extern "C" fn wrapped_callback(
            ctx: JsContextRef,
            object: JsObjectRef,
            property_name: JsStringRef,
            value: JsValueRef,
            exception: *mut JsValueRef,
        ) -> bool {
            CALLBACK.with(|inner| {
                let func = inner.get().unwrap();
                let res = func(
                    JsContext::from_rf(ctx),
                    JsObject::from_rf(object),
                    JsString::from_rf(property_name),
                    {
                        if value == null_mut() {
                            None
                        } else {
                            Some(JsValue::from_rf(value))
                        }
                    },
                );

                match res {
                    Ok(o) => o,
                    Err(e) => {
                        // write the exception
                        unsafe {
                            *exception = e.rf;
                        }
                        false
                    }
                }
            })
        }

        let callback = unsafe { mem::transmute::<_, ObjectPropertySetCallback<'static>>(callback) };
        CALLBACK.with(|inner| unsafe { inner.set(callback).unwrap_unchecked() });

        self.setProperty = Some(wrapped_callback);
        self.jscore__dealloc_set.replace(CALLBACK);

        self
    }

    #[inline(always)]
    pub fn with_attributes(mut self, attributes: JsPropertyAttributes) -> Self {
        self.attributes = attributes.bits();
        self
    }

    #[must_use]
    #[inline(always)]
    pub fn build(self) -> ReusableJsStaticValue {
        ReusableJsStaticValue {
            sv: JsStaticValue {
                name: self.name,
                getProperty: self.getProperty,
                setProperty: self.setProperty,
                attributes: self.attributes,
            },
            _dealloc_get: self.jscore__dealloc_get,
            _dealloc_set: self.jscore__dealloc_set,
        }
    }
}

pub struct ReusableJsStaticValue {
    sv: JsStaticValue,
    _dealloc_get: Option<LocalKey<OnceCell<ObjectPropertyGetCallback<'static>>>>,
    _dealloc_set: Option<LocalKey<OnceCell<ObjectPropertySetCallback<'static>>>>,
}

impl ReusableJsStaticValue {
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const JsStaticValue {
        &raw const self.sv
    }
}

#[allow(nonstandard_style)]
pub struct JsStaticFunctionBuilder {
    /// A null-terminated UTF-8 string that contains the property’s name.
    name: *const c_char,

    /// A callback to invoke when calling the property as a function.
    callAsFunction: JsObjectCallAsFunctionCallback,

    /// A set of property attributes to give to the property.
    attributes: jscore_sys::JsPropertyAttributes,

    jscore__caf: Option<LocalKey<OnceCell<ObjectCallAsFunctionCallback<'static>>>>,
}

impl JsStaticFunctionBuilder {
    pub fn new<K: AsRef<str>>(function_name: K) -> Self {
        let s = CString::new(function_name.as_ref()).expect("found null byte in class_name");

        // note: requires taking ownership back
        Self {
            name: s.into_raw(),
            callAsFunction: None,
            attributes: kJSPropertyAttributeNone,
            jscore__caf: None,
        }
    }

    pub fn with_call_as_function(mut self, callback: ObjectCallAsFunctionCallback) -> Self {
        thread_local! {
            static CALLBACK: OnceCell<ObjectCallAsFunctionCallback<'static>> = OnceCell::new();
        };
        unsafe extern "C" fn wrapped_callback(
            ctx: JsContextRef,
            function: JsObjectRef,
            this: JsObjectRef,
            arg_count: usize,
            arguments: *const JsValueRef,
            exception: *mut JsValueRef,
        ) -> JsValueRef {
            CALLBACK.with(|inner| {
                let func = inner.get().unwrap();
                let res = func(
                    JsContext::from_rf(ctx),
                    JsObject::from_rf(function),
                    JsObject::from_rf(this),
                    arg_count,
                    unsafe {
                        // JsValue is repr(transparent)
                        mem::transmute(slice::from_raw_parts(arguments, arg_count))
                    },
                );

                match res {
                    Ok(o) => o.rf,
                    Err(e) => {
                        // write the exception
                        unsafe {
                            *exception = e.rf;
                        }
                        null_mut()
                    }
                }
            })
        }

        // essentially, as long as 'ctx (the context) is still alive,
        // it can call this function; when it dies, this function dies
        let callback =
            unsafe { mem::transmute::<_, ObjectCallAsFunctionCallback<'static>>(callback) };
        CALLBACK.with(|inner| unsafe { inner.set(callback).unwrap_unchecked() });

        self.callAsFunction = Some(wrapped_callback);
        self.jscore__caf.replace(CALLBACK);

        self
    }

    #[inline(always)]
    pub fn with_attributes(mut self, attributes: JsClassAttributes) -> Self {
        self.attributes = attributes.bits();
        self
    }

    #[inline(always)]
    pub fn build(self) -> ReusableJsStaticFunction {
        ReusableJsStaticFunction {
            sf: JsStaticFunction {
                name: self.name,
                callAsFunction: self.callAsFunction,
                attributes: self.attributes,
            },
            _caf: self.jscore__caf,
        }
    }
}

pub struct ReusableJsStaticFunction {
    sf: JsStaticFunction,
    _caf: Option<LocalKey<OnceCell<ObjectCallAsFunctionCallback<'static>>>>,
}

impl ReusableJsStaticFunction {
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const JsStaticFunction {
        &raw const self.sf
    }
}

impl Drop for ReusableJsStaticFunction {
    fn drop(&mut self) {
        let _ = unsafe { CString::from_raw(self.sf.name.cast_mut()) };
    }
}

/// A structure that contains properties and callbacks that define a type of object.
#[allow(nonstandard_style)]
pub struct JsClassDefinitionBuilder<'ctx> {
    _phantom: PhantomData<&'ctx ()>,

    /// A JavaScript class to set as the class’s parent class.
    parentClass: JsClassRef,

    /// A null-terminated UTF-8 string that contains the class’s name.
    className: *const c_char,

    /// The version of the class definition structure.
    version: c_int,

    /// A set of class attributes to give to the class.
    ///
    /// Combine multiple attributes by performing the logical OR operation.
    attributes: jscore_sys::JsClassAttributes,

    /// An array that contains the class’s statically declared value properties.
    staticValues: *const JsStaticValue,

    /// An array that contains the class’s statically declared function properties.
    staticFunctions: *const JsStaticFunction,

    /// The callback for creating the object.
    initialize: JsObjectInitializeCallback,

    /// The callback for preparing the object for garbage collection.
    finalize: JsObjectFinalizeCallback,

    /// The callback for determining whether an object has a property.
    hasProperty: JsObjectHasPropertyCallback,

    /// The callback for getting a property’s value.
    getProperty: JsObjectGetPropertyCallback,

    /// The callback for setting a property’s value.
    setProperty: JsObjectSetPropertyCallback,

    /// The callback for deleting a property.
    deleteProperty: JsObjectDeletePropertyCallback,

    /// The callback for collecting the names of an object’s properties.
    getPropertyNames: JsObjectGetPropertyNamesCallback,

    /// The callback for calling an object as a function.
    callAsFunction: JsObjectCallAsFunctionCallback,

    /// The callback for checking whether an object is an instance of a particular type.
    hasInstance: JsObjectHasInstanceCallback,

    /// The callback for using an object as a constructor.
    callAsConstructor: JsObjectCallAsConstructorCallback,

    /// The callback for converting an object to a particular JavaScript type.
    convertToType: JsObjectConvertToTypeCallback,

    jscore__static_values: Option<Box<[ReusableJsStaticValue]>>,
}

impl<'ctx> JsClassDefinitionBuilder<'ctx> {
    /// Creates a new JavaScript class definition.
    #[inline]
    pub const fn new(version: c_int) -> Self {
        Self {
            _phantom: PhantomData,
            parentClass: null_mut(),
            className: null_mut(),
            version,
            attributes: kJSClassAttributeNone,
            staticValues: null_mut(),
            staticFunctions: null_mut(),
            initialize: None,
            finalize: None,
            hasProperty: None,
            getProperty: None,
            setProperty: None,
            deleteProperty: None,
            getPropertyNames: None,
            callAsFunction: None,
            hasInstance: None,
            callAsConstructor: None,
            convertToType: None,
            jscore__static_values: None,
        }
    }

    #[inline]
    pub fn with_parent_class(mut self, parent_class: &'ctx JsClass) -> Self {
        self.parentClass = parent_class.as_ptr();
        self
    }

    #[inline]
    pub fn with_class_name<K: AsRef<str>>(mut self, class_name: K) -> Self {
        let s = CString::new(class_name.as_ref()).expect("found null byte in class_name");

        // note: requires taking ownership back
        self.className = s.into_raw();
        self
    }

    #[inline]
    pub fn with_attributes(mut self, attributes: JsClassAttributes) -> Self {
        self.attributes = attributes.bits();
        self
    }

    #[inline]
    pub fn with_static_values<const N: usize>(
        mut self,
        static_values: [ReusableJsStaticValue; N],
    ) -> Self {
        let values = Box::new(static_values);
        self.jscore__static_values = Some(values);
        self
    }

    #[inline]
    pub fn with_static_functions(mut self, static_functions: ReusableJsStaticFunction) -> Self {
        self.staticFunctions = static_functions.as_ptr();
        self
    }

    #[inline(always)]
    pub fn build(self) -> ReusableJsClassDefinition {
        ReusableJsClassDefinition {
            def: JsClassDefinition {
                version: self.version,
                attributes: self.attributes,
                className: self.className,
                parentClass: self.parentClass,
                staticValues: self.staticValues,
                staticFunctions: self.staticFunctions,
                initialize: self.initialize,
                finalize: self.finalize,
                hasProperty: self.hasProperty,
                getProperty: self.getProperty,
                setProperty: self.setProperty,
                deleteProperty: self.deleteProperty,
                getPropertyNames: self.getPropertyNames,
                callAsFunction: self.callAsFunction,
                callAsConstructor: self.callAsConstructor,
                hasInstance: self.hasInstance,
                convertToType: self.convertToType,
            },
            _static_values: self.jscore__static_values,
        }
    }
}

pub struct ReusableJsClassDefinition {
    def: JsClassDefinition,
    _static_values: Option<Box<[ReusableJsStaticValue]>>,
}

impl ReusableJsClassDefinition {
    #[inline(always)]
    const fn as_ptr(&self) -> *const JsClassDefinition {
        &raw const self.def
    }
}

impl Drop for ReusableJsClassDefinition {
    fn drop(&mut self) {
        if self.def.className != null_mut() {
            let _ = unsafe { CString::from_raw(self.def.className.cast_mut()) };
        }

        // if let Some(sv) = self.static_values.take() {
        //     drop(sv);
        // }
    }
}
