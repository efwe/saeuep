use gtk::{Application, ApplicationWindow, Builder, Button, EmojiChooser, glib};
use gtk::glib::clone;
use gtk::prelude::*;

fn main() -> glib::ExitCode {
    let application = Application::new(
        Some("de.efwe.saeuep"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run()
}

pub fn build_ui(application: &Application) {
    let ui_src = include_str!("saeuep.ui");
    let builder = Builder::new();
    builder
        .add_from_string(ui_src)
        .expect("Could not read UI from 'saeuep.ui'");

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let emoji_button: Button = builder.object("emoji_button").expect("Couldn't get emoji_button");
    emoji_button.set_label("😀");
    let emoji_chooser: EmojiChooser = EmojiChooser::builder()
        .build();

    emoji_chooser.connect_emoji_picked(|_emoji_chooser, emoji| {
        print!("{}", emoji);
    });


    emoji_button.set_child(Some(&emoji_chooser));
    emoji_button.connect_clicked(clone!(@weak emoji_chooser => move |_| {
        emoji_chooser.popup();
        emoji_chooser.present();
    }));
    window.present();
}
