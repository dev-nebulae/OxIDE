use egui::{Context, SystemTheme};

pub fn style_app(ctx: &Context) {
    // dark theme
    ctx.set_visuals(egui::Visuals::dark());

    // change font
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Inconsolata Nerd Font".to_owned(),
    );
}
