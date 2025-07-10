pub mod path;

#[derive(Clone, Copy)]
pub enum Task {
    Path(path::Data),
}
