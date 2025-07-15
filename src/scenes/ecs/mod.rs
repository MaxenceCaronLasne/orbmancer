use crate::{
    Force,
    error::Error,
    save::Save,
    scenes::{Scene, ecs::peg::PegsPlugin},
};
//use agb::input::ButtonController;
use agb::rng::RandomNumberGenerator;
use agb::{display::object::Object, fixnum::num, hash_map::HashMap};
use ball::BallPlugin;
use bevy::prelude::*;
use const_random::const_random;
use physics::PhysicsPlugin;
use render::RenderPlugin;

mod ball;
mod peg;
mod physics;
mod render;

pub fn main(gba: &mut agb::Gba, _save: &mut Save) -> Result<Scene, Error> {
    let mut gfx = gba.graphics.get();
    //let mut input = ButtonController::new();
    //let mut timers = gba.timers.timers();
    //crate::bench::init(&mut timers);

    let mut app = App::new();

    let rng = RandomNumberGenerator::new_with_seed([
        const_random!(u32),
        const_random!(u32),
        const_random!(u32),
        const_random!(u32),
    ]);

    app.insert_non_send_resource(rng);

    app.add_plugins(PhysicsPlugin {
        static_walls: (0, 0, 160, 160),
        kinematic_walls: (0, 0, 160, 160),
        gravity: Force::new(num!(0), num!(9.8)),
        kinematic_radius: 4,
        static_radius: 3,
        field_radius: 20,
    });
    app.add_plugins(RenderPlugin);
    app.add_plugins(BallPlugin {
        initial_position: (53, 10),
    });
    app.add_plugins(PegsPlugin::new_random(50, 20, 140, 40, 120));

    loop {
        app.update();

        let mut oam = app
            .world_mut()
            .get_non_send_resource_mut::<HashMap<Entity, Object>>()
            .unwrap();

        let mut frame = gfx.frame();

        for (_, o) in oam.iter_mut() {
            o.show(&mut frame);
        }

        frame.commit();
    }
}
