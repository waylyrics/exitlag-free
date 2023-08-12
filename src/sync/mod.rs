use std::cell::RefCell;

use mpris::{Player, PlayerFinder, TrackID};

use crate::lyric::LyricOwned;


mod utils;
mod lyric;
mod player;
mod cache;

pub use lyric::register_lyric_display;
pub use player::register_mpris_sync;
pub use player::register_sigusr1_disconnect;
pub use player::register_action_disconnect;
pub use player::register_action_connect;
pub use player::register_action_reload_lyric;

pub(crate) use player::list_avaliable_players;

thread_local! {
    static PLAYER: RefCell<Option<Player>> = RefCell::new(None);
    static PLAYER_FINDER: RefCell<PlayerFinder> = RefCell::new(PlayerFinder::new().unwrap());
    static LYRIC: RefCell<(LyricOwned, LyricOwned)> = RefCell::new((LyricOwned::None, LyricOwned::None));
    static TRACK_PLAYING_PAUSED: RefCell<(Option<TrackID>, bool)> = RefCell::new((None, false));
}
