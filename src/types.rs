mod alias;
mod config;
mod constant;
mod dir;
mod document;
mod match_;
mod property;
mod selectfont;
mod value;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntOrRange {
    Int(Int),
    Range(Int, Int),
}

pub use self::{
    alias::*, config::*, constant::*, dir::*, document::*, match_::*, property::*, selectfont::*,
    value::*,
};
