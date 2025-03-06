use std::ops::Deref;

use eframe::egui;

use crate::Opts;
pub struct FnlockGuiApp {
  cmd_opts: Opts,
  states: Vec<String>,
}

impl FnlockGuiApp {
  fn default_with_opts(cmd_opts: Opts) -> Self {
    Self {
      cmd_opts,
      states: vec!["".to_owned(), "".to_owned(), "".to_owned(), "".to_owned()],
    }
  }
}

impl eframe::App for FnlockGuiApp {
  fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        // File menu
        ui.menu_button("File", |ui| {
          if ui.button("Quit").clicked() {
            std::process::exit(0)
          }
        });
        ui.menu_button("Help", |ui| if ui.button("About").clicked() {})
      });
      // [2025-03-06T23:52:51+08:00] tried out table following table example
      // (https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/table_demo.rs)
      // it turned out it requires extra dependency `egui_extra`. The table code
      // is heavily bloated and wrapped in a bad-taste way (I have to say the
      // entire code of egui is quite messy, weird and possibly developed from a
      // temporary toy project, however the good news is that the necessary part
      // is simple). When it ran, it threw out something error like
      //
      //     [2025-03-06T15:49:41Z WARN  egui_glow::painter] You forgot to call
      //     destroy() on the egui glow painter. Resources will leak!
      //
      //     error: process didn't exit successfully:
      //     `target\debug\fnlock.exe --gui` (exit code: 101)
      //
      // Similarly someone raised the issue
      // (https://github.com/emilk/egui/discussions/1386), however it remained
      // unresolved when commenting.
      //
      // So I have to switch back to the grid approach. And from this deep dive
      // experience, I had a new understanding about Rust and its ecosystem. It
      // was quiet difficult to say Rust would become more popular in the
      // future, but I guess eventually it might evolve to another version of
      // OCaml or Lisp, probably more popular than these two but never popular
      // than C-based family and Python. The most important thing of a
      // programming language is just one thing: SIMPLE ENOUGH (I mean from any
      // view, including architecture, semantics, etc), because programming
      // language is essentially a TOOL between human and machine. A tool should
      // be simple, well balanced between reliabilty and flexibility. Human
      // beings should control tool instead of being controlled by tool.
      // However, this is never in part of Rust design and culture. The Rust
      // community seem making it as a pyramid rather than an ecosystem.

      // [2025-03-07T00:18:25+08:00] This piece of code is smelly, though I
      // would like to make it simpler.
      egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::Grid::new("dashboard")
          .num_columns(3)
          .spacing([40.0, 2.0])
          .striped(true)
          .show(ui, |ui| {
            ui.strong("keyboard");
            ui.strong("state");
            ui.strong("action");
            ui.end_row();

            for device_id in 0..3 {
              ui.add(egui::Label::new("keyboard1"));
              let states = &mut self.states;
              ui.add(egui::Label::new(&states[device_id]));
              ui.horizontal_centered(|ui| {
                let button_lock = egui::Button::new("Lock");
                let button_unlock = egui::Button::new("Unlock");
                ui.add_enabled(false, button_lock);
                if ui.add(button_unlock).clicked() {
                  states[device_id] = String::from("Changed");
                }
              });
              ui.end_row();
            }

            ui.add(egui::Label::new("keyboard2"));
            if ui.button("Lock").clicked() {}
            if ui.button("Unlock").clicked() {}
            ui.end_row();
          });
      });
    });
  }
}

pub fn gui_main(opts: Opts) {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
    ..Default::default()
  };
  let _ = eframe::run_native(
    "Fnlock",
    options,
    Box::new(|cc| Ok(Box::new(FnlockGuiApp::default_with_opts(opts)))),
  );
  ()
}
