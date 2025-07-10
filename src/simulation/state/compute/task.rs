pub mod path;

#[derive(Clone, Debug)]
pub enum Task {
    Path(path::Data),
}
