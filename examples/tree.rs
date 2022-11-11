use guiedit::sfml::graphics::RenderWindow;
use guiedit_derive::{Inspectable, TreeNode};
use sfml::{
    graphics::{Color, RenderTarget},
    window::{Event, Key, Style},
};

fn main() {
    let mut window = RenderWindow::new((800, 600), "Inspection", Style::CLOSE, &Default::default());

    struct NonInspectable;

    #[derive(TreeNode, Inspectable)]
    struct InternalStruct {
        stuff: u32,
        noninspectable: NonInspectable,
    }

    #[derive(TreeNode, Inspectable)]
    struct State {
        foo: InternalStruct,
        vec: Vec<InternalStruct>,
        integer: i32,
        string: String,
        color: Color,
    }

    let mut state = State {
        foo: InternalStruct {
            stuff: 0,
            noninspectable: NonInspectable,
        },
        vec: vec![
            InternalStruct {
                stuff: 1,
                noninspectable: NonInspectable,
            },
            InternalStruct {
                stuff: 2,
                noninspectable: NonInspectable,
            },
        ],
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
