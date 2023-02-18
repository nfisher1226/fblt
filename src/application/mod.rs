mod imp;

use {
    adw::{
        gtk::{
            self,
            gio::{self, ApplicationFlags},
            glib::{self, Object},
            prelude::ToValue,
        },
        prelude::*,
        subclass::prelude::*,
    },
    fretboard_layout::Config,
    pango::FontDescription,
};

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, adw::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

impl Application {
    #[must_use]
    pub fn new() -> Self {
        Object::builder()
            .property("application-id", &Some("org.hitchhiker_linux.gfret"))
            .property("flags", &ApplicationFlags::HANDLES_OPEN)
            .property("register-session", &true.to_value())
            .build()
    }

    #[must_use]
    pub fn config(&self) -> Config {
        let settings = &self.imp().settings;
        let units = settings
            .get::<String>("units")
            .parse()
            .expect("Could not parse units from string");
        let border = settings.get::<f64>("border-width");
        let line_weight = settings.get::<f64>("line-weight");
        let fretline_color = settings
            .get::<String>("fretline-color")
            .parse::<gdk::RGBA>()
            .expect("The string could not be parsed into an RGBA struct")
            .into();
        let fretboard_color = settings
            .get::<String>("fretboard-color")
            .parse::<gdk::RGBA>()
            .expect("The string could not be parsed into an RGBA struct")
            .into();
        let centerline_color = Some(
            settings
                .get::<String>("centerline-color")
                .parse::<gdk::RGBA>()
                .expect("The string could not be parsed into an RGBA struct")
                .into(),
        );
        let font = settings.get::<String>("specs-font");
        let font = Some(FontDescription::from_string(&font).into());
        Config {
            units,
            border,
            line_weight,
            fretline_color,
            fretboard_color,
            centerline_color,
            font,
        }
    }
}
