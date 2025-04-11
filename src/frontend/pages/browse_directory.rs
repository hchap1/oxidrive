use std::path::PathBuf;

use iced::widget::{Column, text};
use iced::{Element, Task};

use crate::frontend::application::Page;
use crate::frontend::message::Message;
use crate::frontend::widget::OxidriveWidget;

pub struct BrowseDirectoryPage {
    dir: PathBuf,
    dir_bar: String,
    contents: Vec<PathBuf>
}

impl BrowseDirectoryPage {
    pub fn new(dir: PathBuf) -> Self {
        let dir_bar = dir.to_string_lossy().to_string();
        Self {
            dir,
            dir_bar,
            contents: Vec::new()
        }
    }
}

impl Page for BrowseDirectoryPage {
fn view(&self) -> Element<'_, Message> {
        let header = OxidriveWidget::search_bar("PATH", &self.dir_bar)
            .on_input(Message::ChangeSearchBar)
            .on_paste(Message::ChangeSearchBar)
            .on_submit(Message::SubmitSearchBar);

        let mut column = Column::new();
        for item in &self.contents {
            if let Some(widget) = OxidriveWidget::dir_entry_widget(item.as_path()) {
                column = column.push(
                    match item.is_dir() {
                        true => widget.on_press(Message::BrowseDirectory(item.clone())),
                        false => widget
                    }
                );
            }
        }

        OxidriveWidget::window(
            Column::new()
                .push(header)
                .push(
                    OxidriveWidget::scrollable(
                        column
                    )
                )
                .into()
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DirectoryEntry(entry) => {
                self.contents.push(entry);
                Task::none()
            }

            Message::ChangeSearchBar(content) => {
                self.dir_bar = content;
                Task::none()
            }

            Message::SubmitSearchBar => {
                let new_path = PathBuf::from(&self.dir_bar);
                
                if new_path.is_dir() {
                    self.dir = new_path;
                    self.dir_bar = self.dir.canonicalize().unwrap().to_string_lossy().to_string();
                } else if new_path.is_file() {
                    self.dir = new_path.parent().unwrap().to_path_buf();
                    self.dir_bar = self.dir.canonicalize().unwrap().to_string_lossy().to_string();
                } else {
                    self.dir_bar = self.dir.canonicalize().unwrap().to_string_lossy().to_string();
                    return Task::none();
                }

                self.contents.clear();
                Task::done(Message::ReadDirectory(self.dir.clone()))
            }

            _ => Task::none()
        }
    }
}
