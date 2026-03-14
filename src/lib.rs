//! WebKit JavaScriptCore bindings, using Bun's patched version.
//!
//! ```rust
//! use jscore::prelude::*;
//!
//! // Create a new context group for managing contexts.
//! let group = JsContextGroup::new();
//!
//! // Create the global context for JS interactions
//! let global = group.create_global_context();
//! let ctx = global.as_context();
//!
//! // Write a script
//! let content = JsString::new("'hello from js!'");
//! let script = Script::builder().script(&content).build();
//!
//! // Evaluate the script
//! let result = script.evaluate(ctx).expect("failed to run script");
//! let result_str = result
//!     .to_string_copy(ctx)
//!     .unwrap()
//!     .to_rust_string()
//!     .unwrap();
//!
//! println!("result: {result_str}");
//! ```

mod class;
mod context;
mod interfaces;
mod object;
mod script;
mod string;
mod r#type;
mod value;

/// Essentials for safely interacting with JavaScriptCore.
pub mod prelude {
    pub use crate::class::*;
    pub use crate::context::*;
    pub use crate::object::*;
    pub use crate::script::*;
    pub use crate::string::*;
    pub use crate::r#type::*;
    pub use crate::value::*;

    pub use crate::interfaces::*;
}
