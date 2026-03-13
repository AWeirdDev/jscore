use std::{ffi::CStr, mem::ManuallyDrop};

use jscore_sys::*;

/// Represents a Javascript string.
#[derive(Debug)]
pub struct JsString {
    pub(crate) rf: Option<JsStringRef>,
}

impl JsString {
    #[inline]
    pub fn new<K: AsRef<str>>(data: K) -> Self {
        Self::new_from_str(data.as_ref())
    }

    pub fn new_from_char(data: char) -> Self {
        let chars: JsChars = data.into();
        Self {
            rf: Some(unsafe { js_string_create_with_characters(chars.get_ptr(), chars.len()) }),
        }
    }

    /// Creates a JavaScript string from a buffer of Unicode characters.
    pub fn new_from_string(data: String) -> Self {
        let chars: JsChars = data.into();
        Self {
            rf: Some(unsafe { js_string_create_with_characters(chars.get_ptr(), chars.len()) }),
        }
    }

    pub fn new_from_str(data: &str) -> Self {
        let chars: JsChars = data.into();
        Self {
            rf: Some(unsafe { js_string_create_with_characters(chars.get_ptr(), chars.len()) }),
        }
    }

    /// Create an empty string.
    pub fn new_empty() -> Self {
        Self {
            rf: Some(unsafe { js_string_create_with_characters(JsChars::new().get_ptr(), 0) }),
        }
    }

    /// Releases a JavaScript string.
    ///
    /// Requires ownership.
    pub fn release(mut self) -> bool {
        if let Some(rf) = self.rf.take() {
            unsafe {
                js_string_release(rf);
            }
            true
        } else {
            false
        }
    }

    /// Releases a JavaScript String without ownership checks.
    pub unsafe fn release_unchecked(&self) -> bool {
        if let Some(rf) = self.rf {
            unsafe {
                js_string_release(rf);
            }
            true
        } else {
            false
        }
    }

    /// Returns the number of Unicode characters in a JavaScript string.
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { js_string_get_length(self.as_ptr()) }
    }

    /// Returns a pointer to the Unicode character buffer that serves as the backing
    /// store for a JavaScript string.
    ///
    /// # Returns
    /// A pointer to the Unicode character buffer that serves as the backing store of
    /// string, which the system deallocates when it deallocates string.
    #[inline(always)]
    pub fn chars_ptr(&self) -> *const u16 {
        unsafe { js_string_get_characters_ptr(self.as_ptr()) }
    }

    /// Converts a JavaScript string into a null-terminated UTF-8 string,
    /// and copies the result into an external byte buffer.
    ///
    /// # Returns
    /// The number of bytes the system writes into buffer (including the
    /// null-terminator byte).
    pub fn get_utf8_cstring(&self, buf: *mut i8, buf_size: usize) -> usize {
        unsafe { js_string_get_utf8c_string(self.as_ptr(), buf, buf_size) }
    }

    /// Returns the maximum number of bytes a JavaScript string uses when you
    /// convert it into a null-terminated UTF-8 string.
    #[inline]
    pub fn get_max_utf8_cstring_size(&self) -> usize {
        unsafe { js_string_get_maximum_utf8c_string_size(self.as_ptr()) }
    }

    /// Copies the bytes and converts to Rust String.
    ///
    /// # Returns
    /// `Some(...)` if successful; `None` if failed.
    pub fn to_rust_string(&self) -> Option<String> {
        let size = self.get_max_utf8_cstring_size();
        let mut buf = vec![0; size];
        self.get_utf8_cstring(buf.as_mut_ptr(), size);

        let cstr = unsafe { CStr::from_ptr(buf.as_ptr() as *const i8) };
        cstr.to_str().ok().map(|s| s.to_owned())
    }

    /// Converts to a pointer.
    ///
    /// # Panics
    /// Panics when the string has been released.
    #[inline(always)]
    pub const fn as_ptr(&self) -> JsStringRef {
        if let Some(ptr) = self.rf {
            ptr
        } else {
            core::panic!("data is released");
        }
    }
}

impl ToString for JsString {
    fn to_string(&self) -> String {
        self.to_rust_string()
            .expect("failed to convert javascript string to rust string")
    }
}

impl From<String> for JsString {
    fn from(value: String) -> Self {
        Self::new_from_string(value)
    }
}

impl From<char> for JsString {
    fn from(value: char) -> Self {
        Self::new_from_char(value)
    }
}

impl From<&str> for JsString {
    fn from(value: &str) -> Self {
        Self::new_from_str(value)
    }
}

#[repr(transparent)]
struct JsChars {
    buf: ManuallyDrop<Vec<u16>>,
}

impl JsChars {
    #[inline(always)]
    const fn new() -> Self {
        Self {
            buf: ManuallyDrop::new(vec![]),
        }
    }

    #[inline(always)]
    fn get_ptr(&self) -> *const u16 {
        self.buf.as_ptr()
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.buf.len()
    }
}

impl From<char> for JsChars {
    fn from(value: char) -> Self {
        let mut buf = vec![0u16; 2];
        value.encode_utf16(&mut buf);
        Self {
            buf: ManuallyDrop::new(buf),
        }
    }
}

impl From<String> for JsChars {
    fn from(value: String) -> Self {
        Self {
            buf: ManuallyDrop::new(value.encode_utf16().collect::<Vec<_>>()),
        }
    }
}

impl From<&'_ str> for JsChars {
    fn from(value: &str) -> Self {
        Self {
            buf: ManuallyDrop::new(value.encode_utf16().collect::<Vec<_>>()),
        }
    }
}
