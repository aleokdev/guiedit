use guiedit_derive::Inspectable;
use sfml::{
    graphics::{Color, RenderTarget},
    window::{Event, Key, Style},
};

fn main() {
    let mut window =
        guiedit::RenderWindow::new((800, 600), "Inspection", Style::CLOSE, &Default::default());

    #[derive(Inspectable, Clone, Copy, Default)]
    struct BgColor {
        r: u8,
        g: u8,
        b: u8,
    }

    #[derive(Inspectable, Clone, Copy)]
    enum BgColorKind {
        Black,
        Gray(u8),
        Color { color: BgColor },
    }

    impl Into<Color> for BgColor {
        fn into(self) -> Color {
            Color::rgb(self.r, self.g, self.b)
        }
    }

    impl Into<Color> for BgColorKind {
        fn into(self) -> Color {
            match self {
                BgColorKind::Black => Color::BLACK,
                BgColorKind::Gray(gray) => Color::rgb(gray, gray, gray),
                BgColorKind::Color { color } => color.into(),
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
