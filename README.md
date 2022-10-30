# guiedit
`guiedit` (provisional name) is a Rust library for easily adding a developer GUI to any graphical application.


## Goal
The goal of this crate is to be able to change a few lines of code in an existing or new codebase and instantly get an editor viewport, an object inspector, graphical gizmos, and even state loading/saving & hot code reloading.

## Tutorial
TODO; Check examples for now


## Progress
| Symbol | Meaning |
| ------ | ------- |
| ✅     | Done; implemented |
| ☑️      | Partial implementation |
| 🚧     | Work-in-progress  |
| ⌛     | Planned; Queued   |

### Editor & Common Features
|   Feature     |   Status  |
| ------------- | --------- |
| Inspector with support for `Inspectable` objects | ✅ |
| `#[derive(Inspectable)]` for enums & structs | ✅ |
| `Inspectable` impl for std & core types | 🚧 |
| Hot code reloading | ⌛ |
| Graphical gizmo support | ⌛ |

### [`sfml`](https://github.com/jeremyletang/rust-sfml) Integration
|   Feature     |   Status  |
| ------------- | --------- |
| Forwarding all user rendering to offscreen texture | ✅ |
| Capturing events from the editor and relaying them to user-side | ☑️ |
| Object inspection via UI | ✅ |
| `#[derive(Inspectable)]` for enums & structs | ✅ |
| `Inspectable` impl for SFML types | 🚧 |
| Graphical gizmos for `Drawable`s | ⌛ |

