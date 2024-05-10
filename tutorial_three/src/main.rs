use cursive::traits::*;
use cursive::views::Button;
use cursive::views::Dialog;
use cursive::views::DummyView;
use cursive::views::EditView;
use cursive::views::LinearLayout;
// use cursive::views::ResizedView;
use cursive::views::SelectView;
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    // this window will start very small if the list is empty
    // let select = SelectView::<String>::new();

    // let select = ResizedView::with_fized_size((10, 5), SelectView::<String>::new());

    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("name_list")
        .fixed_size((10, 5));

    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select)
                .child(DummyView)
                .child(buttons),
        )
        .title("Select a profile"),
    );

    siv.run();
}

fn add_name(s: &mut Cursive) {
    /*
    s.add_layer(
        Dialog::around(EditView::new().fixed_width(10))
            .title("Enter a new name")
            .button("Ok", |s| {})
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
    */

    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("name_list", |view: &mut SelectView<String>| {
            view.add_item_str(name);
        });
        s.pop_layer();
    }

    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(ok)
                .with_name("name_input")
                .fixed_width(10),
        )
        .title("Enter a new name")
        .button("Ok", |s| {
            let name = s
                .call_on_name("name_input", |view: &mut EditView| view.get_content())
                .unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    );
}

fn delete_name(s: &mut Cursive) {
    let mut select = s.find_name::<SelectView<String>>("name_list").unwrap();

    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}

fn on_submit(s: &mut Cursive, name: &str) {
    s.pop_layer();
    s.add_layer(
        Dialog::text(format!("Name: {}\nAwesome: yes", name))
            .title(format!("{}'s info", name))
            .button("Quit", Cursive::quit),
    );
}
