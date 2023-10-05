use glib_macros::clone;
use gtk::{
    gio::SimpleAction,
    glib::{self, clone::Downgrade, VariantTy},
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
    Application,
};
use tracing::{error, info, warn};
mod utils;

use crate::{
    app::{self, dialog::show_dialog},
    lyric_providers::{default_search_query, LyricOwned},
    sync::{
        interop::{reset_lyric_labels, update_lyric},
        lyric::cache::update_lyric_cache,
        search_window, TrackState, LYRIC, PLAYER, PLAYER_FINDER, TRACK_PLAYING_STATE,
    },
};

pub fn register_action_disconnect(app: &Application) {
    let action = SimpleAction::new("disconnect", None);
    action.connect_activate(|_, _| {
        PLAYER.set(None);
    });
    app.add_action(&action);
}

pub fn register_sigusr1_disconnect() {
    glib::unix_signal_add_local(libc::SIGUSR1, move || {
        PLAYER.set(None);
        Continue(true)
    });
}

// TODO: code cleanup
pub fn register_action_search_lyric(app: &Application, wind: &app::Window, trigger: &str) {
    let action = SimpleAction::new("search-lyric", None);
    let cache_lyrics = wind.imp().cache_lyrics.get();
    action.connect_activate(move |_, _| {
        // Get current playing track
        let query_default = TRACK_PLAYING_STATE.with_borrow(|TrackState { metainfo, .. }| {
            metainfo.as_ref().map(|track| {
                let artists = if let Some(artists) = track.artists.as_ref() {
                    artists.iter().map(|s| s.as_str()).collect()
                } else {
                    vec![]
                };
                default_search_query(
                    track.album.as_ref().map(|s| s.as_str()).unwrap_or_default(),
                    &artists,
                    &track.title,
                )
            })
        });

        let window = search_window::Window::new(query_default.as_deref(), cache_lyrics);
        window.present();
    });
    app.add_action(&action);

    utils::bind_shortcut("app.search-lyric", wind, trigger);
}

pub fn register_action_refetch_lyric(app: &Application, wind: &app::Window, trigger: &str) {
    let action = SimpleAction::new("refetch-lyric", None);
    let window = Downgrade::downgrade(wind);
    action.connect_activate(move |_, _| {
        info!("cleaned current lyric");
        TRACK_PLAYING_STATE.with_borrow(|TrackState { metainfo, .. }| {
            let (Some(metainfo), Some(window)) = (metainfo, window.upgrade()) else {
                return;
            };

            if let Err(err) = update_lyric(metainfo, &window, true) {
                show_dialog(
                    Some(&window),
                    &format!("cannot refetch lyric: {err:?}"),
                    gtk::MessageType::Error,
                )
            }
        });
    });
    app.add_action(&action);

    utils::bind_shortcut("app.refetch-lyric", wind, trigger);
}

pub fn register_action_remove_lyric(app: &Application, wind: &app::Window) {
    let action = SimpleAction::new("remove-lyric", None);
    let cache_lyrics = wind.imp().cache_lyrics.get();
    action.connect_activate(clone!(@weak wind as window => move |_, _| {
        // Clear current lyric
        LYRIC.with_borrow_mut(|(origin, translation)| {
            *origin = LyricOwned::LineTimestamp(vec![]);
            *translation = LyricOwned::None;
        });
        // Update cache
        if cache_lyrics {
            TRACK_PLAYING_STATE.with_borrow(|TrackState{ cache_path, ..}| {
                if let Some(cache_path) = cache_path {
                    update_lyric_cache(cache_path);
                }
            });
        }
        // Remove current lyric inside window
        reset_lyric_labels(&window);
        info!("removed lyric");
    }));
    app.add_action(&action);
}

pub fn register_action_connect(app: &Application) {
    let connect = SimpleAction::new("connect", Some(VariantTy::STRING));
    connect.connect_activate(|_, player_id| {
        let Some(player_id) = player_id.and_then(|p| p.str()) else {
            warn!("did not received string paramter for action \'app.connect\'");
            return;
        };
        PLAYER_FINDER.with_borrow(|player_finder| {
            if let Ok(player) = player_finder.find_by_name(player_id) {
                PLAYER.set(Some(player));
            } else {
                error!("cannot connect to: {player_id}");
            }
        });
    });
    app.add_action(&connect);
}
