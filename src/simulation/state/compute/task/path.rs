pub mod kind;
pub mod data;

pub use kind::Kind;

pub enum Path {
    Local(data::Local),
    Regional(data::Regional),
}
