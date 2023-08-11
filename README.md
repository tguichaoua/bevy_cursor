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

The following cursor informations are available via the `CursorInfo` resource:

- The entity id of the window on which the cursor is currently;
- The entity if of the camera on which the cursor is currently;
- The position of the cursor on the window (logical position);
- The 2D world position of the cursor (if the feature `2d` is enabled);
- The [ray](https://docs.rs/bevy/0.11.0/bevy/index.html) emitted by the cursor through the camera (if the feature `3d` is enabled);

## Example

```rust ,no_run
use bevy::prelude::*;
use bevy_cursor::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CursorInfoPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, print_cursor_position)
        .run();
}

fn setup(mut commands: Commands) {
    // A camera is required to compute the world position of the cursor
    commands.spawn(Camera2dBundle::default());
}

fn print_cursor_position(cursor: Res<CursorInfo>) {
    if let Some(position) = cursor.position() {
        info!("Cursor position: {position:?}");
    } else {
        info!("The cursor is not in any window");
    }
}
```

## Features

- `2d` opt-in the computation of the world position of the cursor.
- `3d` opt-in the computation of the [ray](https://docs.rs/bevy/0.11.0/bevy/index.html) emitted by the cursor through the camera (disabled by default).

## Bevy compatible version

| bevy | bevy_cursor |
| ---- | ----------- |
| 0.11 | 0.1         |
