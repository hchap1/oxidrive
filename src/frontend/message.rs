use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    BrowseDirectory(PathBuf),
    ReadDirectory(PathBuf),
    DirectoryEntry(PathBuf),    // A single result has been loaded
    None,

    ChangeSearchBar(String),
    SubmitSearchBar
}
