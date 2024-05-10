use cursive::views::TextView;

fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    // no dialog/border, has drop shadow - label with shadow
    siv.add_layer(TextView::new("Hello cursive! Press <q> to quit."));

    siv.run();
}
