use eframe::egui;

pub struct FnlockGuiApp {
  unlock: bool,
}

impl Default for FnlockGuiApp {
  fn default() -> Self {
    Self { unlock: false }
  }
}

impl eframe::App for FnlockGuiApp {
  fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("My egui Application");
    });
  }
}

pub fn gui_main() {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
    ..Default::default()
  };
  let _ = eframe::run_native(
    "My egui App",
    options,
    Box::new(|cc| Ok(Box::<FnlockGuiApp>::default())),
  );
  ()
}
