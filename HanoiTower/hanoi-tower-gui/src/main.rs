///////////////////////////////////////////////////////////////////////////////

use bevy::{
    app::{App, Startup, Update},
    DefaultPlugins,
};
use hanoi_tower_solver::{debug, hanoi_general_rec, hanoi_simple_iter, hanoi_simple_rec};

use crate::camera::pan_orbit_camera;

///////////////////////////////////////////////////////////////////////////////

mod camera;
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

    let i = 4;
    let j = 4;
    println!("{} pegs with {} disks", i, j);
    let f: Vec<u32> = (0..j).rev().collect();
    let mut rods = vec![];
    for n in 0..i {
        rods.push(vec![]);
    }
    rods[0] = f.clone();
    hanoi_general_rec(
        j.try_into().unwrap(),
        0,
        &mut rods,
        0,
        i - 1,
        (1..i - 1).collect(),
    );
    println!();

    println!();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, systems::setup)
        .add_systems(Update, pan_orbit_camera)
        .run();
}

///////////////////////////////////////////////////////////////////////////////
