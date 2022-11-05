use guiedit::{inspectable::Inspectable, tree::TreeNode};
use guiedit_derive::{Inspectable, TreeNode};
use sfml::{
    graphics::{Color, RenderTarget},
    window::{Event, Key, Style},
};

fn main() {
    let mut window =
        guiedit::RenderWindow::new((800, 600), "Inspection", Style::CLOSE, &Default::default());

    struct CustomNode;

    impl Inspectable for CustomNode {}
    impl TreeNode for CustomNode {
        fn inspect_child(&mut self, _this_id: u64, _search_id: u64, ui: &mut egui::Ui) {}

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
                let label = egui::SelectableLabel::new(
                    matches!(*selected, Some(i) if i == id),
                    egui::RichText::new(name).strong(),
                );
                if ui.add(label).clicked() {
                    *selected = Some(id);
                }
                ui.add_enabled_ui(false, |ui| ui.small("CustomNode"));
            })
            .body(|ui| self.tree_ui(id, selected, ui));
        }

        fn tree_ui(&mut self, _id: u64, _selected: &mut Option<u64>, ui: &mut egui::Ui) {
            ui.label("Custom content!");
            use egui::plot::{Line, PlotPoints};
            let n = 128;
            let line_points: PlotPoints = (0..=n)
                .map(|i| {
                    use std::f64::consts::TAU;
                    let x = egui::remap(i as f64, 0.0..=n as f64, -TAU..=TAU);
                    [x, x.sin()]
                })
                .collect();
            let line = Line::new(line_points);
            egui::plot::Plot::new("example_plot")
                .height(32.0)
                .data_aspect(1.0)
                .show(ui, |plot_ui| plot_ui.line(line));
        }
    }

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        window.display_and_inspect(&mut CustomNode)
    }
}
