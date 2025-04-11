use std::sync::mpsc::{self, Receiver, Sender};
use std::{path::PathBuf, pin::Pin, task::Waker};
use std::thread::spawn;

use iced::futures::Stream;
use tokio::{fs::read_dir, task::JoinHandle};

use crate::frontend::message::Message;
use crate::backend::sync::{AM, sync, desync};

pub struct StreamContents {
    _search_task: Option<JoinHandle<()>>,
    manager: Option<std::thread::JoinHandle<()>>,
    waker: AM<Option<Waker>>,
    items: AM<Vec<PathBuf>>
}

fn manage_items(
    receiver: Receiver<Option<PathBuf>>,
    waker: AM<Option<Waker>>,
    items: AM<Vec<PathBuf>>
) {
    loop {
        let incoming = match receiver.recv() {
            Ok(incoming) => incoming,
            Err(_) => break
        };

        let entry = match incoming {
            Some(entry) => entry,
            None => break
        };

        {
            let mut items = desync(&items);
            items.push(entry);
        }

        {
            let waker = desync(&waker);
            if let Some(waker) = waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    {
        let waker = desync(&waker);
        if let Some(waker) = waker.as_ref() {
            waker.wake_by_ref();
        }
    }
}

async fn discover_items(
    dir: PathBuf, send: Sender<Option<PathBuf>>
) {
    println!("[ASYNC-DISCOVER] Starting to search: {dir:?}");

    let mut entries = match read_dir(dir).await {
        Ok(entries) => entries,
        Err(_) => return
    };

    while let Ok(entry) = entries.next_entry().await {
        if let Some(entry) = entry {
            println!("[ASYNC-DISCOVER] Discovered {:?}", entry.path());
            let _ = send.send(Some(entry.path()));
        } else {
            break;
        }
    }
    let _ = send.send(None);
    println!("[ASYNC-DISCOVER] Thread ending");
}

impl StreamContents {
    pub fn new(dir: PathBuf) -> Self {
        let waker = sync(None);
        let waker_clone = waker.clone();
        let items = sync(Vec::new());
        let items_clone = items.clone();
        let (send, recv) = mpsc::channel();

        Self {
            _search_task: Some(
                tokio::spawn(
                    discover_items(
                        dir, send
                    )
                )
            ),
            manager: Some(spawn(move || {
                manage_items(recv, waker_clone, items_clone)
            })),
            items,
            waker
        }
    }
}

impl Stream for StreamContents {
    type Item = Message;

    fn poll_next(self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Option<Message>> {
        {
            let mut waker = self.waker.lock().unwrap();
            if waker.is_none() {
                *waker = Some(context.waker().clone());
            }
        }

        let mut items = self.items.lock().unwrap();
        match items.pop() {
            Some(item) => std::task::Poll::Ready(Some(Message::DirectoryEntry(item))),
            None => {
                if let Some(handle) = self.manager.as_ref() {
                    if handle.is_finished() {
                        println!("[ASYNC-DISCOVER] Task finished. Ending");
                        std::task::Poll::Ready(None)
                    } else {
                        println!("[ASYNC-DISCOVER] Task pending");
                        std::task::Poll::Pending
                    }
                } else {
                    println!("[ASYNC-DISCOVER] No handle. Ending");
                    std::task::Poll::Ready(None)
                }
            }
        }
    }
}
