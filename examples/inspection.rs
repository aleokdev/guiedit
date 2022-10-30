use guiedit_derive::Inspectable;
use sfml::{
    graphics::{Color, RenderTarget},
    window::{Event, Key, Style},
};

fn main() {
    let mut window =
        guiedit::RenderWindow::new((800, 600), "Inspection", Style::CLOSE, &Default::default());

    #[derive(Inspectable, Clone, Copy)]
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
        window.display_and_inspect(&mut bg_color)
    }
}
