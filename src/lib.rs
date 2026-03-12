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
