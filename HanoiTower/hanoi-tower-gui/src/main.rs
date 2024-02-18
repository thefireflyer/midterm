///////////////////////////////////////////////////////////////////////////////

use std::vec;

use bevy::{
    app::{App, Startup, Update},
    DefaultPlugins,
};
use hanoi_tower_solver::{debug, hanoi_general_rec, hanoi_simple_iter, hanoi_simple_rec};

use crate::{camera::pan_orbit_camera, resources::TowerConfig, systems::button_system};

///////////////////////////////////////////////////////////////////////////////

mod camera;
mod components;
mod resources;
mod systems;

///////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut o = vec![4, 3, 2, 1];
    let mut a = vec![];
    let mut t = vec![];
    hanoi_simple_rec(4, &mut o, &mut a, &mut t, 0, &|o, a, t, d| {
        debug(o, a, t, d)
    });

    println!();

    let mut o = vec![4, 3, 2, 1];
    let mut a = vec![];
    let mut t = vec![];
    hanoi_simple_iter(&mut o, &mut a, &mut t);

    println!();
    App::new()
        .insert_resource(TowerConfig {
            number_of_disks: 3,
            number_of_tower: 3,
            running: false,
            speed: 1.0,
            moves: vec![],
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, systems::setup)
        .add_systems(Update, (pan_orbit_camera, button_system))
        .run();
}

///////////////////////////////////////////////////////////////////////////////
