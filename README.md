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

**Bevy Cursor is a [`bevy`] plugin to track information about the cursor.**

---

This plugin will track information about the position of the cursor, the window, and the camera that contains it and compute the position of the pointed point in the world position system.

## Example

```rust
use bevy::prelude::*;
use bevy_cursor::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TrackCursorPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, print_cursor_position)
        .run();
}

fn setup(mut commands: Commands) {
    // A camera is required to compute the world position of the cursor
    commands.spawn(Camera2d);
}

fn print_cursor_position(cursor: Res<CursorLocation>) {
    if let Some(position) = cursor.position() {
        info!("Cursor position: {position:?}");
    } else {
        info!("The cursor is not in any window");
    }
}
```

## Features

- `2d` opt-in the computation of the world position of the cursor.
- `3d` opt-in the computation of the [ray] emitted by the cursor through the camera.

## Bevy compatible version

| bevy | bevy_cursor |
| ---- | ----------- |
| 0.15 | 0.5         |
| 0.14 | 0.4         |
| 0.13 | 0.3         |
| 0.12 | 0.2         |
| 0.11 | 0.1         |

[`bevy`]: https://github.com/bevyengine/bevy
[ray]: https://docs.rs/bevy/0.15.0/bevy/math/struct.Ray3d.html
