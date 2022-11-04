use guiedit::{
    inspect,
    inspectable::{Inspectable, InspectableNode, TreeElement, ValueWrapper},
};
use guiedit_derive::{Inspectable, TreeElement};
use sfml::{
    graphics::{Color, RenderTarget},
    window::{Event, Key, Style},
};

fn main() {
    let mut window =
        guiedit::RenderWindow::new((800, 600), "Inspection", Style::CLOSE, &Default::default());

    #[derive(TreeElement, Inspectable)]
    struct InternalStruct {
        stuff: u32,
    }

    #[derive(TreeElement, Inspectable)]
    struct State {
        foo: InternalStruct,
        integer: i32,
        string: String,
        color: Color,
    }

    let mut state = State {
        foo: InternalStruct { stuff: 0 },
        integer: 0,
        string: "Hello World!".to_owned(),
        color: Color::BLACK,
    };

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
        window.display_and_inspect(&mut state)
    }
}
