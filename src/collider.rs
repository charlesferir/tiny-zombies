use crate::map::TILE_PIXEL_SIZE;
use crate::player::Player;
use crate::utils::cursor_position;
use crate::DebugMode;
use bevy::{app::AppExit, prelude::*};
use bevy_prototype_debug_lines::*;

pub struct ColliderPugin;

#[derive(Component)]
pub struct Collider;

pub trait FromTransform {
    fn from_transform(transform: &Transform) -> Self;
}

impl FromTransform for Rectangle {
    #[inline]
    fn from_transform(transform: &Transform) -> Self {
        Self {
            pos: transform.translation.truncate(),
            width: transform.scale.x * TILE_PIXEL_SIZE as f32,
            height: transform.scale.y * TILE_PIXEL_SIZE as f32,
        }
    }
}
#[derive(Debug)]
pub struct Rectangle {
    pos: Vec2,
    width: f32,
    height: f32,
}

#[derive(Component, Debug)]
pub struct DebugRay;

impl Plugin for ColliderPugin {
    fn build(&self, app: &mut App) {
        // app.add_system(handle_collisions.after(move_player_system));
        app.add_system(collider_debug_box);
        app.add_system(player_debug_box.after(collider_debug_box));
        // app.add_system(mouse_vs_rect);
    }
}

fn rect_vs_rect(r1: &Rectangle, r2: &Rectangle) -> bool {
    r1.pos.x - r1.width / 2. < r2.pos.x + r2.width / 2.
        && r1.pos.x + r1.width / 2. > r2.pos.x - r2.width / 2.
        && r1.pos.y - r1.height / 2. < r2.pos.y + r2.height / 2.
        && r1.pos.y + r1.height / 2. > r2.pos.y - r2.height / 2.
}

fn point_vs_rect(point: &Vec2, rect: &Rectangle) -> bool {
    point.x >= rect.pos.x - rect.width / 2.
        && point.y >= rect.pos.y - rect.height / 2.
        && point.x < rect.pos.x + rect.width / 2.
        && point.y < rect.pos.y + rect.height / 2.
}

fn collider_debug_box(
    collider_query: Query<(&Transform, &Collider)>,
    mut shapes: ResMut<DebugShapes>,
    debug_mode: Res<DebugMode>,
    window: Query<&mut Window>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if !debug_mode.on {
        return;
    }

    let mut hover = vec![];

    for (collider_transform, _) in &collider_query {
        let collide_box = Rectangle::from_transform(collider_transform);
        if let Some(cursor_pos) = cursor_position(&window, &mut app_exit_events) {
            if point_vs_rect(&cursor_pos, &collide_box) {
                hover.push(collide_box);
                continue;
            }
        }
        add_debug_rect(&mut shapes, &collide_box, Color::WHITE);
    }

    for collide_box in hover {
        add_debug_rect(&mut shapes, &collide_box, Color::YELLOW);
    }
}

fn player_debug_box(
    collider_query: Query<(&Transform, &Collider), Without<Player>>,
    player_query: Query<(&Transform, &Collider), With<Player>>,
    mut shapes: ResMut<DebugShapes>,
    debug_mode: Res<DebugMode>,
) {
    if !debug_mode.on {
        return;
    }

    if let Ok((player_hit_box, _)) = player_query.get_single() {
        let player_hit_box = Rectangle::from_transform(player_hit_box);
        for (collider_transform, _) in &collider_query {
            let collide_box = Rectangle::from_transform(collider_transform);
            if rect_vs_rect(&player_hit_box, &collide_box) {
                add_debug_rect(&mut shapes, &collide_box, Color::RED);
                add_debug_rect(&mut shapes, &player_hit_box, Color::RED);
            }
        }
    }
}

fn add_debug_rect(shapes: &mut ResMut<DebugShapes>, rect: &Rectangle, color: Color) {
    shapes
        .rect()
        .position(Vec3::new(rect.pos.x, rect.pos.y, 0.))
        .size(Vec2::new(rect.width, rect.height))
        .color(color);
}
