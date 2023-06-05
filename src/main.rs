use gtk::{Application, ApplicationWindow, Builder, EmojiChooser, glib, Label};
use gtk::glib::{clone, MainContext};
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
    let emoji_label : Label= builder.object("emoji_label").expect("Couldn't get emoji_label");
    let emoji_chooser : EmojiChooser= builder.object("emoji_chooser").expect("Couldn't get emoji_chooser");
    emoji_label.set_text("(x)");
    emoji_chooser.connect_emoji_picked(clone!(@weak emoji_label => move |_emoji_chooser, emoji| {
        emoji_label.set_text(emoji);
        println!("emoji {:#?}", emoji);
    }));
    // show the emoji chooser without blocking the main thread


    // show a window with the emoji chooser





    //let main_context = MainContext::default();
    // The main loop executes the asynchronous block
    // main_context.spawn_local(clone!(@weak emoji_chooser => async move {
    //     emoji_chooser.show();
    //     //emoji_chooser.present();
    //     //emoji_chooser.popup();
    //     println!("chooser {}", emoji_chooser);
    //     let widget = emoji_chooser.default_widget();
    //     println!("widget {:#?}", widget);
    //     let pointing = emoji_chooser.pointing_to();
    //     println!("pointing to {}", pointing.0);
    // }));
    emoji_chooser.show();
    window.present();
}
