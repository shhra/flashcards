mod database;
mod org;
mod ui;
mod files;

use ui::App;

fn main() {
    let mut app = App::default();
    app.init();
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}
