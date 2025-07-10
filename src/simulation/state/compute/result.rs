pub mod path;

#[derive(Clone, Debug)]
pub enum Result {
    Path(path::Data),
}