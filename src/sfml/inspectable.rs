use sfml::{
    graphics::Color,
    system::{Vector2, Vector3},
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
