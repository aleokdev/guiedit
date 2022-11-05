use std::hash::Hasher;

use crate::inspectable::Inspectable;

pub trait TreeNode: Inspectable {
    /// Searches for an object with the ID given in this element and its children, and calls its
    /// inspect_ui function if it is found.
    // TODO: Return controlflow for more performant search
    fn inspect_child(&mut self, this_id: u64, search_id: u64, ui: &mut egui::Ui);

    fn tree_ui_outside(
        &mut self,
        name: &str,
        id: u64,
        selected: &mut Option<u64>,
        ui: &mut egui::Ui,
    ) {
        egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            ui.make_persistent_id(id),
            false,
        )
        .show_header(ui, |ui| {
            if ui
                .selectable_label(matches!(*selected, Some(i) if i == id), name)
                .clicked()
            {
                *selected = Some(id);
            }
        })
        .body(|ui| self.tree_ui(id, selected, ui));
    }

    fn tree_ui(&mut self, _id: u64, _selected: &mut Option<u64>, _ui: &mut egui::Ui) {}
}

impl<T: TreeNode, const X: usize> TreeNode for [T; X] {
    fn inspect_child(&mut self, this_id: u64, search_id: u64, ui: &mut egui::Ui) {
        if this_id == search_id {
            self.inspect_ui(ui);
        } else {
            for (i, element) in self.into_iter().enumerate() {
                let mut hasher = std::collections::hash_map::DefaultHasher::default();
                hasher.write_u64(this_id);
                hasher.write_u64(i as u64);
                // Depth-first search
                element.inspect_child(this_id, search_id, ui);
            }
        }
    }

    fn tree_ui(&mut self, id: u64, selected: &mut Option<u64>, ui: &mut egui::Ui) {
        for (i, element) in self.into_iter().enumerate() {
            let mut hasher = std::collections::hash_map::DefaultHasher::default();
            hasher.write_u64(id);
            hasher.write_u64(i as u64);
            element.tree_ui_outside(&i.to_string(), hasher.finish(), selected, ui);
        }
    }
}

impl<T: TreeNode> TreeNode for Vec<T> {
    fn inspect_child(&mut self, this_id: u64, search_id: u64, ui: &mut egui::Ui) {
        if this_id == search_id {
            self.inspect_ui(ui);
        } else {
            for (i, element) in self.into_iter().enumerate() {
                let mut hasher = std::collections::hash_map::DefaultHasher::default();
                hasher.write_u64(this_id);
                hasher.write_u64(i as u64);
                // Depth-first search
                element.inspect_child(this_id, search_id, ui);
            }
        }
    }

    fn tree_ui(&mut self, id: u64, selected: &mut Option<u64>, ui: &mut egui::Ui) {
        for (i, element) in self.into_iter().enumerate() {
            let mut hasher = std::collections::hash_map::DefaultHasher::default();
            hasher.write_u64(id);
            hasher.write_u64(i as u64);
            element.tree_ui_outside(&i.to_string(), hasher.finish(), selected, ui);
        }
    }
}

impl<T: TreeNode + ?Sized> TreeNode for &mut T {
    fn inspect_child(&mut self, this_id: u64, search_id: u64, ui: &mut egui::Ui) {
        (*self).inspect_child(this_id, search_id, ui)
    }

    fn tree_ui(&mut self, id: u64, selected: &mut Option<u64>, ui: &mut egui::Ui) {
        (*self).tree_ui(id, selected, ui)
    }
}
