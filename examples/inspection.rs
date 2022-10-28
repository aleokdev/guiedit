use guiedit::inspectable::ValueWrapper;
use sfml::{
    graphics::{Color, RenderTarget},
    window::{Event, Key, Style},
};

fn main() {
    let mut window =
        guiedit::RenderWindow::new((800, 600), "Inspection", Style::CLOSE, &Default::default());

    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

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

        window.clear(Color::rgb(r, g, b));
        window.display_and_inspect(&mut [
            ValueWrapper {
                name: "r",
                value: &mut r,
            },
            ValueWrapper {
                name: "g",
                value: &mut g,
            },
            ValueWrapper {
                name: "b",
                value: &mut b,
            },
        ])
    }
}
