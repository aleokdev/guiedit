use std::ops::DerefMut;

use sfml::SfBox;

use crate::tree::TreeNode;

impl<T: TreeNode + sfml::SfResource> TreeNode for SfBox<T> {
    fn inspect_child(&mut self, this_id: u64, search_id: u64, ui: &mut egui::Ui) {
        self.deref_mut().inspect_child(this_id, search_id, ui)
    }

    fn tree_ui(&mut self, id: u64, selected: &mut Option<u64>, ui: &mut egui::Ui) {
        self.deref_mut().tree_ui(id, selected, ui);
    }
}
