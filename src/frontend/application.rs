use std::path::PathBuf;

use iced::Element;
use iced::Task;

use crate::frontend::pages::browse_directory::BrowseDirectoryPage;
use crate::frontend::message::Message;

use crate::backend::filemanager::StreamContents;

pub trait Page {
    fn view(&self) -> Element<'_, Message>;
    fn update(&mut self, message: Message) -> Task<Message>;
}

pub struct Application {
    page: Box<dyn Page>
}

impl Application {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            page: Box::new(BrowseDirectoryPage::new(directory))
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ReadDirectory(target) => Task::stream(StreamContents::new(target)),
            Message::BrowseDirectory(target) => {
                self.page = Box::new(BrowseDirectoryPage::new(target.clone()));
                Task::done(Message::ReadDirectory(target))
            }
            page_specific => self.page.update(page_specific)
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        self.page.view()
    }
}
