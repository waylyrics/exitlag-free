mod imp;

use glib::Object;
use gtk::glib::clone;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, ColumnViewColumn};
use gtk::{prelude::*, ListItem};
use tracing::error;

use crate::LYRIC_PROVIDERS;

use crate::app::dialog::show_dialog;
use crate::sync::lyric::cache::update_lyric_cache;
use crate::sync::{TrackState, LYRIC, TRACK_PLAYING_STATE};

glib::wrapper! {
    pub struct ResultObject(ObjectSubclass<imp::ResultObject>);
}

impl ResultObject {
    pub fn new(
        id: String,
        title: String,
        singer: String,
        album: String,
        length: u64,
        provider_idx: usize,
    ) -> Self {
        Object::builder()
            .property("title", title)
            .property("singer", singer)
            .property("album", album)
            .property("length", length)
            .property("id", id)
            .property("provider-idx", provider_idx as u8)
            .build()
    }
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(query_default: Option<&str>, use_cache: bool) -> Self {
        let window: Self = Object::builder().build();
        window.set_title(Some("Search lyric"));
        if let Some(query) = query_default {
            window.imp().input.buffer().set_text(query);
        }
        window.imp().use_cache.set(use_cache);
        window
    }

    fn results(&self) -> gio::ListStore {
        self.imp()
            .results
            .borrow()
            .clone()
            .expect("Could not get results")
    }

    fn setup_results(&self) {
        let model = gio::ListStore::new(ResultObject::static_type());

        self.imp().results.replace(Some(model));

        let selection_model = gtk::SingleSelection::new(Some(self.results()));
        self.imp().result_list.set_model(Some(&selection_model));
    }

    fn setup_ui(&self) {
        let imp = self.imp();

        imp.vbox
            .set_properties(&[("orientation", &gtk::Orientation::Vertical)]);
        imp.vbox.append(&imp.input);
        imp.vbox.append(&imp.result_scrolled_window);
        imp.vbox.append(&imp.set_button);

        imp.result_scrolled_window.set_child(Some(&imp.result_list));
        imp.result_scrolled_window.set_vexpand(true);
        imp.result_scrolled_window
            .set_hscrollbar_policy(gtk::PolicyType::Never);
        imp.result_scrolled_window.set_height_request(300);
        imp.result_scrolled_window.set_width_request(300);

        imp.result_title.set_title(Some("Title"));
        imp.result_list.append_column(&imp.result_title);
        imp.result_title.set_expand(true);
        imp.result_title.set_resizable(true);

        imp.result_singer.set_title(Some("Singer"));
        imp.result_list.append_column(&imp.result_singer);
        imp.result_singer.set_resizable(true);
        imp.result_album.set_title(Some("Album"));
        imp.result_list.append_column(&imp.result_album);
        imp.result_album.set_resizable(true);
        imp.result_length.set_title(Some("Length"));
        imp.result_list.append_column(&imp.result_length);

        imp.input.set_placeholder_text(Some("Enter query..."));
        imp.input
            .set_secondary_icon_name(Some("system-search-symbolic"));
        imp.set_button.set_label("Set as lyric");

        self.set_child(Some(&imp.vbox));
    }

    fn search(&self) {
        let buffer = self.imp().input.buffer();
        let query = buffer.text().to_string();
        if query.is_empty() {
            return;
        }

        let mut results = vec![];
        LYRIC_PROVIDERS.with_borrow(|providers| {
            for (idx, provider) in providers.iter().enumerate() {
                let provider_id = provider.provider_unique_name();
                let tracks = match provider.search_song(&query) {
                    Ok(songs) => songs,
                    Err(e) => {
                        // TODO: to show errors to users in GUI
                        error!("{e} occurs when search {query} on {}", provider_id);
                        continue;
                    }
                };
                for track in tracks {
                    results.push(ResultObject::new(
                        track.id,
                        track.title,
                        track.singer,
                        track.album.unwrap_or_default(),
                        track.length.as_secs(),
                        idx,
                    ));
                }
            }
        });
        self.results().remove_all();
        if results.is_empty() {
            show_dialog(Some(self), "No result found.", gtk::MessageType::Error);
            return;
        }
        for result in results {
            self.results().append(&result);
        }
    }

    fn get_selected_result(&self) -> Option<ResultObject> {
        let selection_model = self
            .imp()
            .result_list
            .model()
            .and_downcast::<gtk::SingleSelection>()
            .expect("Needs to be SingleSelection");
        let selected = selection_model.selected();
        if selected == gtk::INVALID_LIST_POSITION {
            return None;
        }
        let result = self
            .results()
            .item(selected)
            .and_downcast::<ResultObject>()
            .expect("Needs to be ResultObject");
        Some(result)
    }

    fn setup_callbacks(&self) {
        let imp = self.imp();
        imp.input
            .connect_activate(clone!(@weak self as window => move |_| {
                window.search();
            }));

        imp.input
            .connect_icon_release(clone!(@weak self as window => move |_, _| {
                window.search();
            }));

        imp.set_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                let result = window.get_selected_result();
                let Some(result) = result else {
                    return;
                };

                LYRIC_PROVIDERS.with_borrow(|providers| {
                    let provider_idx = result.provider_idx() as usize;
                    if provider_idx >= providers.len() {
                        error!("provider_idx {} is out of range", provider_idx);
                        return;
                    }
                    let provider = providers[provider_idx].as_ref();
                    let song_id = result.id();
                    match provider.query_lyric(&song_id) {
                        Ok(lyric) => {
                            let olyric = provider.get_lyric(&lyric);
                            let tlyric = provider.get_translated_lyric(&lyric);
                            LYRIC.with_borrow_mut(|(origin, translation)| {
                                *origin = olyric;
                                *translation = tlyric;
                            });

                            if window.imp().use_cache.get() {
                                TRACK_PLAYING_STATE.with_borrow(|TrackState {cache_path, ..}| {
                                    if let Some(cache_path) = cache_path {
                                        update_lyric_cache(cache_path);
                                    }
                                });
                            }
                        },
                        Err(e) => {
                            let error_msg = format!("{e} when getting lyric.");
                            error!(error_msg);
                            show_dialog(Some(&window), &error_msg, gtk::MessageType::Error);
                        }
                    }
                })
            }));
    }

    fn setup_factory(&self) {
        let imp = self.imp();
        connect_factory(&imp.result_title, |result| result.title(), true);
        connect_factory(&imp.result_singer, |result| result.singer(), false);
        connect_factory(&imp.result_album, |result| result.album(), true);
        connect_factory(
            &imp.result_length,
            |result| format_length(result.length()),
            false,
        );
    }
}

fn connect_factory(
    column: &ColumnViewColumn,
    get_field: impl 'static + Fn(ResultObject) -> String,
    wrap: bool,
) {
    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        label.set_halign(gtk::Align::Start);
        label.set_wrap(wrap);
        label.set_wrap_mode(gtk::pango::WrapMode::Word);
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&label));
    });
    factory.connect_bind(move |_, list_item| {
        let result_object = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<ResultObject>()
            .expect("Needs to be ResultObject");
        let label = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .child()
            .and_downcast::<gtk::Label>()
            .expect("Needs to be Label");

        label.set_label(&get_field(result_object));
    });

    column.set_factory(Some(&factory));
}

fn format_length(length: u64) -> String {
    let min = length / 60;
    let sec = length % 60;
    format!("{min:02}:{sec:02}")
}
