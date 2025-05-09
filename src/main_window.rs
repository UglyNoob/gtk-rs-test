use adw::prelude::*;
use adw::subclass::prelude::ObjectSubclassIsExt;
use builder::MainWindowBuilder;
use gtk::gio::PropertyAction;
use gtk::{Widget, Window, glib};

use gtk::gio::{self, ActionEntry};

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<main_window_imp::MainWindowImp>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, Window, Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

mod main_window_imp;

impl MainWindow {
    pub fn builder<'a>() -> MainWindowBuilder<'a> {
        MainWindowBuilder::new()
    }

    fn setup_actions(&self) {
        self.add_action_entries([
            ActionEntry::builder("about")
                .activate(|window: &Self, _, _| {
                    window.imp().show_about_dialog();
                })
                .build(),
            ActionEntry::builder("remove-done-tasks")
                .activate(|window: &Self, _, _| {
                    window.imp().remove_done_tasks();
                })
                .build(),
            ActionEntry::builder("add-collection")
                .parameter_type(Some(&String::static_variant_type()))
                .activate(|window: &Self, _, param| {
                    let title = param.and_then(|v| v.get::<String>()).unwrap();

                    window.imp().add_collection(&title);
                })
                .build(),
        ]);
        self.add_action(&PropertyAction::new("filter-mode", self, "filter_mode"));
        self.add_action(&PropertyAction::new(
            "adaptive-preview",
            self,
            "adaptive_preview",
        ));
    }
}

mod builder {
    use gtk::glib::{Object, object::ObjectBuilder};

    use super::MainWindow;

    pub struct MainWindowBuilder<'a> {
        builder: ObjectBuilder<'a, MainWindow>,
    }

    impl<'a> MainWindowBuilder<'a> {
        pub fn new() -> Self {
            Self {
                builder: Object::builder(),
            }
        }
        pub fn application(self, application: &adw::Application) -> Self {
            Self {
                builder: self.builder.property("application", application),
            }
        }
        pub fn build(self) -> MainWindow {
            self.builder.build()
        }
    }
}

mod collection_wizard;
mod task_row;
