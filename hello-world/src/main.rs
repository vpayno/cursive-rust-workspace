use cursive::views::{Dialog, TextView};

fn main() {
    // Creates the cursive root
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button.
    siv.add_layer(
        Dialog::around(TextView::new("Hello World!"))
            .title("Cursive")
            .button("Quit", |s| s.quit()),
    );

    // Starts the event loop.
    siv.run();
}
