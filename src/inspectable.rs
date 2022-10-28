use std::ops::RangeInclusive;

pub trait Inspectable {
    fn inspect_ui(&mut self, ui: &mut egui::Ui);
}

macro_rules! implement_inspectable_for_numeric {
    ($NumericType: ty) => {
        impl Inspectable for $NumericType {
            fn inspect_ui(&mut self, ui: &mut egui::Ui) {
                ui.add(egui::DragValue::new(self));
            }
        }
    };
}
implement_inspectable_for_numeric!(i8);
implement_inspectable_for_numeric!(u8);
implement_inspectable_for_numeric!(i16);
implement_inspectable_for_numeric!(u16);
implement_inspectable_for_numeric!(i32);
implement_inspectable_for_numeric!(u32);
implement_inspectable_for_numeric!(i64);
implement_inspectable_for_numeric!(u64);
implement_inspectable_for_numeric!(isize);
implement_inspectable_for_numeric!(usize);
implement_inspectable_for_numeric!(f32);
implement_inspectable_for_numeric!(f64);

pub struct ClampedValue<'v, T: egui::emath::Numeric> {
    pub range: RangeInclusive<T>,
    pub value: &'v mut T,
    pub speed: f64,
}

impl<T: egui::emath::Numeric> Inspectable for ClampedValue<'_, T> {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        ui.add(
            egui::DragValue::new(self.value)
                .clamp_range(self.range.clone())
                .speed(self.speed),
        );
    }
}

pub struct ValueWrapper<'v, T: Inspectable> {
    pub name: &'static str,
    pub value: &'v mut T,
}

impl<'v, T: Inspectable> Inspectable for ValueWrapper<'v, T> {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(self.name);
            self.value.inspect_ui(ui);
        });
    }
}

impl Inspectable for () {
    fn inspect_ui(&mut self, _ui: &mut egui::Ui) {}
}

impl<'v, T: Inspectable, const X: usize> Inspectable for [T; X] {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        for element in self.into_iter() {
            element.inspect_ui(ui);
        }
    }
}

impl<T: Inspectable + ?Sized> Inspectable for &mut T {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        (*self).inspect_ui(ui)
    }
}
