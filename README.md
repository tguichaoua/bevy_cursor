# Bevy Cursor

[![Latest Version]][crates.io] [![Bevy Tracking]][bevy tracking doc] [![Doc Status]][docs] [![Build Status]][actions]

[Latest Version]: https://img.shields.io/crates/v/bevy_cursor.svg
[crates.io]: https://crates.io/crates/bevy_cursor
[Bevy Tracking]: https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue?labelColor=555555&logo=bevy
[bevy tracking doc]: https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking
[Doc Status]: https://docs.rs/bevy_cursor/badge.svg
[docs]: https://docs.rs/bevy_cursor
[Build Status]: https://github.com/tguichaoua/bevy_cursor/actions/workflows/ci.yml/badge.svg?branch=main
[actions]: https://github.com/tguichaoua/bevy_cursor/actions/workflows/ci.yml

**Bevy Cursor is a [`bevy`](https://github.com/bevyengine/bevy) plugin to track informations about the cursor.**

---

## Example

```rust
use bevy::prelude::*;
use bevy_cursor::prelude::*;


fn main() {
    App::new()
        //
        .add_plugins(DefaultPlugins)
        .add_plugins(CursorInfoPlugin) // Add the plugin
        //
        .add_systems(Update, print_cursor_info)
        .run();
}

fn print_cursor_info(cursor: Res<CursorInfo>) {
    if let Some(position) = cursor.position() {
        info!("Cursor position: {position:?}");
    } else {
        info!("The cursor is not in any window");
    }
}

```

## Bevy compatible version

| bevy | bevy_cursor |
| ---- | ----------- |
| 0.11 | 0.1         |
