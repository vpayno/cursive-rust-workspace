use cursive::views::Dialog;
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    // dialog with thin border with shadow
    siv.add_layer(
        Dialog::text("Press the button!")
            .title("Frist dialog")
            .button("Next", show_next),
    );

    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Did you do the thing?")
            .title("Question 1")
            .button("Yes!", |s| show_answer(s, "I knew it! Well done!"))
            .button("No!", |s| show_answer(s, "I knew you couldn't be trusted!"))
            .button("Uh?", |s| s.add_layer(Dialog::info("Try again!")))
            .button("Nevermind", |s| s.quit()),
    );
}

fn show_answer(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(
        Dialog::text(msg)
            .title("Results")
            .button("Finish", |s| s.quit()),
    );
}
