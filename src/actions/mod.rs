mod keys;
use {
    crate::{Application, Window},
    adw::gtk::{
        gio::SimpleAction,
        glib::{self, clone},
        prelude::*,
    },
    gettext::gettext,
    keys::Keys,
};

pub struct Actions<'a> {
    names: [&'a str; 7],
}

impl<'a> Default for Actions<'a> {
    fn default() -> Self {
        Self {
            names: [
                "open",
                "save",
                "save_as",
                "open_external",
                "preferences",
                "about",
                "quit",
            ],
        }
    }
}

impl<'a> Actions<'a> {
    pub fn add(&self, win: &Window, app: &Application) {
        let keys = Keys::from_file().unwrap_or_default();
        for name in &self.names {
            let action = SimpleAction::new(name, None);
            app.set_accels_for_action(&format!("win.{name}"), &[keys.get(name)]);
            win.add_action(&action);
            match *name {
                "open" => {
                    action.connect_activate(clone!(@weak win => move |_, _| {
                        win.open_file();
                    }));
                }
                "save" => {
                    action.connect_activate(clone!(@weak win => move |_, _| {
                        win.save();
                    }));
                }
                "save_as" => {
                    action.connect_activate(clone!(@weak win => move |_, _| {
                        win.save_as();
                    }));
                }
                "open_external" => {
                    action.connect_activate(clone!(@weak win => move |_, _| {
                        win.open_external();
                    }));
                }
                "preferences" => {
                    action.connect_activate(clone!(@weak win => move |_, _| {
                        win.run_preferences();
                    }));
                }
                "about" => {
                    action.connect_activate(clone!(@strong win, @weak app => move |_, _| {
                        let win = adw::AboutWindow::builder()
                            .application_icon("gfret")
                            .application_name(&env!("CARGO_PKG_NAME").to_uppercase())
                            .comments(&gettext("A tool for lutherie\nBuilt using Rust and Gtk+"))
                            .copyright("©2020 by Nathan Fisher (the JeanG3nie)")
                            .developer_name("Nathan Fisher")
                            .issue_url("https://codeberg.org/jeang3nie/gfret/issues")
                            .license_type(adw::gtk::License::Bsd)
                            .release_notes("<p>Unreleased</p>\
                                <ul>\
                                <li>Move some common code into lib.rs</li>\
                                <li>Create trait `ConvertUnits` to swap imperial and metric values</li>\
                                <li>Move action handling into module</li>\
                                <li>Move keybindings into module</li>\
                                <li>Make keybindings configurable</li>\
                                <li>Depend on libadwaita</li>\
                                <li>Subclass Application from AdwApplication</li>\
                                <li>Subclass Window from AdwWindow</li>\
                                <li>Use adwaita AboutWindow</li>\
                                <li>Subclass PreferencesWindow from AdwPreferencesWindow</li>\
                                <li>Store application state and settings in gschema format</li>\
                                <li>Open previously created files directly instead of using templates</li>\
                                <li>Add AdwToastOverlay to send in app notifications for file save etc.</li>\
                                </ul>"
                            )
                            .version(env!("CARGO_PKG_VERSION"))
                            .website("https://jeang3nie.codeberg.page/gfret/")
                            .application(&app)
                            .transient_for(&win)
                            .build();
                        win.show();
                    }));
                }
                "quit" => {
                    action.connect_activate(clone!(@weak win => move |_, _| {
                        win.close();
                    }));
                }
                _ => unreachable!(),
            }
        }
    }
}
