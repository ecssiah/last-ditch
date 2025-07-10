pub mod data;
pub mod kind;

pub use kind::Kind;

pub enum Path {
    Local(data::Local),
    Regional(data::Regional),
}
