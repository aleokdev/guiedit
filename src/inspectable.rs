use std::{ops::RangeInclusive, path::PathBuf, time::Duration};

pub trait Inspectable {
    fn inspect_ui_outside(&mut self, name: &str, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(name);
            self.inspect_ui(ui);
        });
    }

    fn inspect_ui(&mut self, _ui: &mut egui::Ui) {}
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

impl Inspectable for bool {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(self, "");
    }
}

impl Inspectable for PathBuf {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        // TODO: Allow editing PathBufs
        ui.add_enabled_ui(false, |ui| {
            ui.text_edit_singleline(&mut self.to_string_lossy().into_owned())
        });
    }
}

impl Inspectable for Duration {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        let mut secs = self.as_secs_f64();
        if ui
            .add(egui::DragValue::new(&mut secs).clamp_range(0.0..=core::f64::INFINITY))
            .changed()
        {
            *self = Duration::from_secs_f64(secs);
        }
    }
}

impl Inspectable for String {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        ui.text_edit_singleline(self);
    }
}

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

pub struct ValueWrapper<'v, T> {
    pub name: &'static str,
    pub value: &'v mut T,
}

impl<'v, T: Inspectable> Inspectable for ValueWrapper<'v, T> {
    // TODO: Get rid of this, ValueWrapper is only for TreeElement now
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(self.name);
            self.value.inspect_ui(ui);
        });
    }
}

pub struct ReadOnlyValue<'v, T: Inspectable>(pub &'v mut T);

// TODO: TreeElement for ReadOnlyValue<T>
impl<'v, T: Inspectable> Inspectable for ReadOnlyValue<'v, T> {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        ui.add_enabled_ui(false, |ui| self.0.inspect_ui(ui));
    }
}

// TODO: TreeElement for Option<T>
impl<T: Inspectable + Default> Inspectable for Option<T> {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        match self {
            Some(x) => {
                if ui
                    .horizontal(|ui| {
                        ui.group(|ui| x.inspect_ui(ui));
                        ui.small_button("-").clicked()
                    })
                    .inner
                {
                    *self = None;
                }
            }
            None => {
                if ui.small_button("+").clicked() {
                    *self = Some(Default::default())
                }
            }
        }
    }
}

impl Inspectable for () {}

impl<T: Inspectable> Inspectable for [T] {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        for element in self.into_iter() {
            element.inspect_ui(ui);
        }
    }
}

impl<T: Inspectable, const X: usize> Inspectable for [T; X] {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        self[..].as_mut().inspect_ui(ui)
    }
}

impl<T: Inspectable> Inspectable for Vec<T> {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        self[..].inspect_ui(ui)
    }
}

impl<T: Inspectable + ?Sized> Inspectable for &mut T {
    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
        (*self).inspect_ui(ui)
    }
}

#[macro_export]
macro_rules! inspect {
    ($(($($val: tt)+)),* $(,)?) => {
        &mut [
            $($crate::__inspect_impl!($($val)+)),*
        ]
    }
}

#[macro_export]
macro_rules! __inspect_impl {
    (mut $val: expr) => {
        &mut $crate::inspectable::ValueWrapper {
            name: stringify!($val),
            value: &mut $val,
        } as &mut dyn $crate::tree::TreeNode
    };

    ($val: expr) => {
        &mut $crate::inspectable::ValueWrapper {
            name: stringify!($val),
            value: &mut $crate::inspectable::ReadOnlyValue(&mut $val),
        } as &mut dyn $crate::tree::TreeNode
    };
}

pub use inspect;
