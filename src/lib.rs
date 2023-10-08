#![allow(stable_features)]
#![feature(local_key_cell_methods)]
#![feature(is_some_and)]

use std::{
    cell::RefCell,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use app::Window;
use lyric_providers::LyricProvider;
use qqmusic_rs::QQMusicApi;
use regex::RegexSet;

pub mod app;
pub mod config;
pub mod lyric_providers;
pub mod sync;
pub mod utils;

pub const APP_ID: &str = "io.poly000.waylyrics";

thread_local! {
    pub static CACHE_DIR: RefCell<String> = RefCell::new(String::new());
    pub static THEME_PATH: RefCell<PathBuf> = RefCell::new(PathBuf::new());
    pub static EXCLUDED_REGEXES: RefCell<RegexSet> = RefCell::new(RegexSet::empty());
    pub static MAIN_WINDOW: RefCell<Option<Window>> = RefCell::new(None);
}

pub static CONFIG_HOME: OnceLock<String> = OnceLock::new();

pub static LYRIC_PROVIDERS: OnceLock<Vec<Arc<dyn LyricProvider>>> = OnceLock::new();
pub static QQMUSIC_API_CLIENT: OnceLock<Option<QQMusicApi>> = OnceLock::new();

pub const DEFAULT_TEXT: &str = "Waylyrics";
