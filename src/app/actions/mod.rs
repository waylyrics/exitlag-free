use crate::app::{utils::set_click_pass_through, Window};

use gtk::gio::SimpleAction;
use gtk::{
    prelude::*, subclass::prelude::*, Application, NamedAction, Shortcut, ShortcutController,
    ShortcutTrigger,
};

pub fn register_action_switch_decoration(wind: &Window, switch_decoration_trigger: &str) {
    let action = SimpleAction::new("switch-decoration", None);
    let _wind = Window::downgrade(wind);
    action.connect_activate(move |_, _| {
        if let Some(wind) = _wind.upgrade() {
            wind.set_decorated(!wind.is_decorated());
        }
    });
    wind.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("win.switch-decoration"))
        .trigger(&ShortcutTrigger::parse_string(switch_decoration_trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}

pub fn register_action_reload_theme(app: &Application, wind: &Window, trigger: &str) {
    let action = SimpleAction::new("reload-theme", None);
    action.connect_activate(move |_, _| {
        crate::THEME_PATH.with_borrow(|theme_path| {
            if let Ok(style) = std::fs::read_to_string(theme_path) {
                crate::app::utils::merge_css(&style);
            }
        });
    });
    app.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("app.reload-theme"))
        .trigger(&ShortcutTrigger::parse_string(trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}

pub fn register_action_switch_passthrough(wind: &Window, trigger: &str) {
    let action = SimpleAction::new("switch-passthrough", None);
    let _wind = Window::downgrade(wind);
    action.connect_activate(move |_, _| {
        if let Some(wind) = _wind.upgrade() {
            let clickthrough = !wind.imp().clickthrough.get();
            wind.imp().clickthrough.set(clickthrough);
            set_click_pass_through(&wind, clickthrough);
            wind.present();
        }
    });
    wind.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("win.switch-passthrough"))
        .trigger(&ShortcutTrigger::parse_string(trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}
