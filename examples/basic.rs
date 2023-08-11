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
