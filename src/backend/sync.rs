use std::sync::{Arc, Mutex, MutexGuard};

pub type AM<T> = Arc<Mutex<T>>;
pub fn sync<T>(obj: T) -> AM<T> { Arc::new(Mutex::new(obj)) }
pub fn desync<T>(obj: &AM<T>) -> MutexGuard<T> { obj.lock().unwrap() }
