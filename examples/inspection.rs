use guiedit::sfml::graphics::RenderWindow;
use guiedit::{Inspectable, TreeNode};
use sfml::{
    graphics::{Color, RenderTarget},
    window::{Event, Key, Style},
};

fn main() {
    let mut window = RenderWindow::new((800, 600), "Inspection", Style::CLOSE, &Default::default());

    #[derive(TreeNode, Inspectable, Clone, Copy)]
    struct BgColor {
        color: Color,
    }

    impl Default for BgColor {
        fn default() -> Self {
            Self {
                color: Color::BLACK,
            }
        }
    }

    #[derive(Inspectable, Clone, Copy)]
    enum BgColorKind {
        Black,
        Gray(u8),
        Color { color: BgColor },
    }

    impl TreeNode for BgColorKind {
        fn inspect_child(&mut self, this_id: u64, search_id: u64, ui: &mut egui::Ui) {
            if this_id == search_id {
                self.inspect_ui(ui);
            }
        }
    }

    impl Into<Color> for BgColorKind {
        fn into(self) -> Color {
            match self {
                BgColorKind::Black => Color::BLACK,
                BgColorKind::Gray(gray) => Color::rgb(gray, gray, gray),
                BgColorKind::Color {
                    color: BgColor { color },
                } => color,
            }
        }
    }

    let mut bg_color = BgColorKind::Black;

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

        window.clear(bg_color.into());
        window.display_and_inspect(&mut bg_color);
    }
}
