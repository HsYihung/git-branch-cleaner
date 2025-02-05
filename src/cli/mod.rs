mod ui;

pub use ui::Cli;

#[derive(Debug)]
pub enum DeleteOption {
    MergedBranches,
    StaleBranches,
    AllBranches,
    SingleBranch,
}

#[derive(Debug)]
pub enum DirectoryChoice {
    SelectDirectory(usize),
    ParentDirectory,
    RootDirectory,
    Exit,
}
