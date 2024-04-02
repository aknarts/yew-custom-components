use std::sync::RwLock;

use gloo_storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use tracing::error;

const DARK_KEY: &str = "theme.self";

lazy_static! {
    /// Jwt token read from local storage.
    pub static ref DARK: RwLock<Option<bool>> = {
        LocalStorage::get(DARK_KEY).map_or_else(|_| RwLock::new(None), |dark| RwLock::new(Some(dark)))
    };
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Theme {
    pub dark: bool,
}

impl yew::Reducible for Theme {
    type Action = bool;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        Theme { dark: action }.into()
    }
}

impl Theme {
    pub fn set_dark(&self, dark: bool) {
        LocalStorage::set(DARK_KEY, Some(dark)).expect("failed to set");
        match DARK.write() {
            Ok(mut w) => {
                *w = Some(dark);
            }
            Err(e) => {
                error!("Error setting dark: {:?}", e);
            }
        }
    }

    pub fn get_dark(&self) -> bool {
        match DARK.write() {
            Ok(mut r) => match r.clone() {
                None => {
                    *r = Some(true);
                    true
                }
                Some(d) => d,
            },
            Err(e) => {
                error!("Error getting dark: {:?}", e);
                true
            }
        }
    }

    pub fn toggle_dark(&self) {
        let dark = !self.get_dark();
        self.set_dark(dark);
    }
}
