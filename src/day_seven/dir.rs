use crate::day_seven::file::File;
#[derive(Debug)]
pub struct Dir<'a> {
    pub name: String,
    pub parent: Option<&'a Dir<'a>>,
    pub dirs: Vec<&'a Dir<'a>>,
    pub files: Vec<File>,
}
