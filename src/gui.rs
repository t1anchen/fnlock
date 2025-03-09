use eframe::egui;
use hidapi::HidApi;
use log::info;

use crate::{
  keyboard::{
    find_device_from_deviceinfo, fnlock, get_api_context, is_device_available,
    list_devices, DeviceInfo,
  },
  Opts,
};
pub struct FnlockGuiApp {
  cmd_opts: Opts,
  api: Option<HidApi>,
  names: Vec<String>,
  states: Vec<String>,
  devices: Vec<DeviceInfo>,
  n_devices: usize,
  to_be_lock: bool,
}

impl FnlockGuiApp {
  fn default_with_opts(cmd_opts: Opts) -> Self {
    let devices = list_devices(get_api_context().as_ref());
    let to_be_lock = !cmd_opts.unlock;
    let api = get_api_context();
    Self {
      api,
      n_devices: devices.len(),
      cmd_opts,
      names: devices
        .iter()
        .map(|d| {
          format!(
            "{}/{} ({:04x}/{:04x}, {:04x}, {:04x})",
            d.vendor_name.trim(),
            d.product_name.trim(),
            d.vendor_id,
            d.product_id,
            d.usage,
            d.usage_page
          )
        })
        .collect(),
      states: vec![String::new(); devices.len()],
      devices,
      to_be_lock,
    }
  }
}

impl eframe::App for FnlockGuiApp {
  fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("menubar").show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        // File menu
        ui.menu_button("File", |ui| {
          if ui.button("Quit").clicked() {
            std::process::exit(0)
          }
        });
        ui.menu_button("Help", |ui| if ui.button("About").clicked() {})
      });
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

    egui::CentralPanel::default().show(ctx, |ui| {
      egui::ScrollArea::both().show(ui, |ui| {
        egui::Grid::new("dashboard")
          .num_columns(8)
          .spacing([40.0, 2.0])
          .striped(true)
          .show(ui, |ui| {
            ui.strong("Vendor Name");
            ui.strong("Product Name");
            ui.strong("Vendor ID");
            ui.strong("Product ID");
            ui.strong("Usage");
            ui.strong("Usage Page");
            ui.strong("state");
            ui.strong("action");
            ui.end_row();

            for device_id in 0..self.n_devices {
              ui.add(egui::Label::new(
                self.devices[device_id].vendor_name.clone(),
              ));
              ui.add(egui::Label::new(
                self.devices[device_id].product_name.clone(),
              ));
              ui.add(egui::Label::new(format!(
                "{:04X}",
                self.devices[device_id].vendor_id
              )));
              ui.add(egui::Label::new(format!(
                "{:04X}",
                self.devices[device_id].product_id
              )));
              ui.add(egui::Label::new(format!(
                "{:04X}",
                self.devices[device_id].usage
              )));
              ui.add(egui::Label::new(format!(
                "{:04X}",
                self.devices[device_id].usage_page
              )));
              let states = &mut self.states;
              ui.add(egui::Label::new(&states[device_id]));
              if is_device_available(&self.devices[device_id]) {
                ui.horizontal_centered(|ui| {
                  if ui.button("Lock").clicked() {
                    fnlock(
                      find_device_from_deviceinfo(
                        self.api.as_ref(),
                        &self.devices[device_id],
                      ),
                      true,
                    );
                    self.states[device_id] = "Locked".to_owned();
                    info!(
                      "device {:?} fn key has been locked",
                      self.devices[device_id]
                    );
                  } else if ui.button("Unlocked").clicked() {
                    fnlock(
                      find_device_from_deviceinfo(
                        self.api.as_ref(),
                        &self.devices[device_id],
                      ),
                      false,
                    );
                    self.states[device_id] = "Unlocked".to_owned();
                    info!(
                      "device {:?} fn key has been unlocked",
                      self.devices[device_id]
                    );
                  }
                });
              }
              ui.end_row();
            }
          });
      });
    });
  }
}

pub fn gui_main(opts: Opts) {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
    ..Default::default()
  };
  let _ = eframe::run_native(
    "Fnlock",
    options,
    Box::new(|cc| Ok(Box::new(FnlockGuiApp::default_with_opts(opts)))),
  );
  ()
}
