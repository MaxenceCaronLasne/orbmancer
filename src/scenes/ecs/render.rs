use crate::position::Position;
use agb::{display::object::Object, hash_map::HashMap};
use bevy::prelude::*;

#[derive(Component)]
pub struct Showable;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(HashMap::<Entity, Object>::new());
        app.add_systems(Last, show);
    }
}

fn show(
    query: Query<(Entity, Option<&Position>), With<Showable>>,
    mut oam: NonSendMut<HashMap<Entity, Object>>,
) {
    for (e, maybe_position) in &query {
        if let Some(object) = oam.get_mut(&e)
            && let Some(p) = maybe_position
        {
            object.set_pos((p.0.x.round(), p.0.y.round()));
        }
    }
}

pub fn add(
    world: &mut World,
    entity: Entity,
    sprite: &'static agb::display::object::Sprite,
) {
    let mut oam = world
        .get_non_send_resource_mut::<HashMap<Entity, Object>>()
        .unwrap();

    oam.insert(entity, Object::new(sprite));
}
