pub mod data;

pub use data::Data;

pub enum Path {
    Local(data::Local),
    Regional(data::Regional),
}