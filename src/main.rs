mod constants;

use ::cursive::views::{Dialog, TextView};
use cursive::{
    theme::{Color, ColorStyle, Palette, Theme},
    views::{Button, LinearLayout},
};

fn main() {
    let mut siv = cursive::default();

    let mut theme = Theme::terminal_default();
    theme
        .palette
        .set_color("Background", Color::TerminalDefault);
    theme.palette.set_color("Primary", constants::WHITE);
    theme.palette.set_color("TitlePrimary", constants::PINK);
    siv.set_theme(theme);

    // let title = TextView::new(constants::OTHELLO)
    //     .style();

    let mut title = TextView::new(constants::OTHELLO);
    title.set_style(ColorStyle::front(constants::PINK));

    let mut player_one_chip = TextView::new(constants::CHIP_OUTLINE);
    player_one_chip.set_style(ColorStyle::front(constants::GREEN));

    let linear_layout = LinearLayout::vertical()
        .child(title)
        .child(player_one_chip)
        .child(Button::new("Ok", |s| s.quit()));

    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(linear_layout);

    siv.run();
}
