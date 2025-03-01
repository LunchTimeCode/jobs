use rocket::tokio::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

type LockedState = Arc<Mutex<State>>;

pub struct _State {
    state: LockedState,
}

impl Default for _State {
    fn default() -> Self {
        _State {
            state: Arc::new(Mutex::new(State::default())),
        }
    }
}

impl _State {
    pub async fn get(&self) -> rocket::tokio::sync::MutexGuard<'_, State> {
        self.state.lock().await
    }
}

#[derive(Debug, Clone, Default)]
pub struct State {
    urls: HashMap<String, String>,
}

impl State {
    pub async fn get_html(&mut self, url: &str) -> &String {
        if !self.urls.contains_key(url) {
            let res = get_url(url).await;
            self.urls.insert(url.to_string(), res);
        }
        self.urls.get(url).unwrap()
    }
}

pub async fn get_url(url: &str) -> String {
    reqwest::get(url).await.unwrap().text().await.unwrap()
}

pub fn initial_state() -> _State {
    _State::default()
}
