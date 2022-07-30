#![feature(box_syntax)]
#![feature(once_cell)]
#![feature(hasher_prefixfree_extras)]

pub use pest::error::Error;
pub use pest::error::{ErrorVariant, LineColLocation};

pub use self::parser::ParseContext;
pub use self::re0::Rule;

pub mod ast;
mod parser;
mod re0;
pub mod value;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
