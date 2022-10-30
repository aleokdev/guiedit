use std::ops::DerefMut;

use sfml::{
    audio::SoundBuffer,
    graphics::{Color, Texture},
    system::{Vector2, Vector3},
    SfBox,
};

use crate::inspectable::Inspectable;

impl Inspectable for Color {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        let mut color = egui::Color32::from_rgba_premultiplied(self.r, self.g, self.b, self.a);

        ui.color_edit_button_srgba(&mut color);

        [self.r, self.g, self.b, self.a] = color.to_array();
    }
}

impl<T: egui::emath::Numeric> Inspectable for Vector2<T> {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut self.x));
                ui.add(egui::DragValue::new(&mut self.y));
            })
        });
    }
}

impl<T: egui::emath::Numeric> Inspectable for Vector3<T> {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut self.x));
                ui.add(egui::DragValue::new(&mut self.y));
                ui.add(egui::DragValue::new(&mut self.z));
            })
        });
    }
}

impl Inspectable for SoundBuffer {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        // TODO: Better impl
        ui.label(format!(
            "SoundBuffer; {:.1}s long",
            self.duration().as_seconds()
        ));
    }
}

impl<T: Inspectable + sfml::SfResource> Inspectable for SfBox<T> {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        self.deref_mut().inspect_ui(ui);
    }
}

impl Inspectable for Texture {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        ui.image(
            egui::TextureId::User(self.native_handle() as u64),
            egui::Vec2::new(self.size().x as f32, self.size().y as f32),
        );
    }
}
