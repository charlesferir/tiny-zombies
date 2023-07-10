use bevy::{app::AppExit, prelude::*};

pub fn cursor_position(
    window: &Query<&mut Window>,
    app_exit_events: &mut EventWriter<AppExit>,
) -> Option<Vec2> {
    let window = match window.get_single() {
        Ok(window) => window,
        Err(_) => {
            error!("No window left");
            app_exit_events.send(AppExit);
            return None;
        }
    };

    if let Some(mut pos) = window.cursor_position() {
        pos.x -= window.width() / 2.;
        pos.y -= window.height() / 2.;
        return Some(pos);
    }
    None
}
