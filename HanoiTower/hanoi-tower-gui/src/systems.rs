///////////////////////////////////////////////////////////////////////////////

use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use hanoi_tower_solver::hanoi_general_rec;

use crate::{
    camera::PanOrbitCamera,
    components::{Disk, Disks, Tower},
    resources::TowerConfig,
};

///////////////////////////////////////////////////////////////////////////////

const TOWER_INC: &str = "TowerInc";
const TOWER_DEC: &str = "TowerDec";
const TOWER_DISPLAY: &str = "TowerDisplay";
const DISK_INC: &str = "DiskInc";
const DISK_DEC: &str = "DiskDec";
const DISK_DISPLAY: &str = "DiskDisplay";
const SPEED_INC: &str = "SpeedInc";
const SPEED_DEC: &str = "SpeedDec";
const SPEED_DISPLAY: &str = "SpeedDisplay";

const START_STOP: &str = "StartStop";

///////////////////////////////////////////////////////////////////////////////

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
    asset_server: Res<AssetServer>,
) {
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Plane::from_size(10.0).into()),
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::GRAY,
    //         ..default()
    //     }),
    //     ..default()
    // });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 3.5, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            radius: 5.0,
            ..Default::default()
        },
    ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0),
        point_light: PointLight {
            intensity: 600000.,
            range: 100.,
            ..default()
        },
        ..default()
    });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_content: bevy::ui::AlignContent::End,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Px(100.),
                        width: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(2.)),
                        align_self: bevy::ui::AlignSelf::End,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(60.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: bevy::prelude::BackgroundColor(Color::rgb(
                                    0.1, 0.1, 0.1,
                                )),
                                ..default()
                            },
                            Name::new(TOWER_INC),
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "+",
                                TextStyle {
                                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font: default(),
                                    font_size: 50.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(60.0),
                                    height: Val::Px(60.0),
                                    // border: UiRect::all(Val::Px(5.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                // border_color: BorderColor(Color::BLACK),
                                background_color: bevy::prelude::BackgroundColor(Color::rgb(
                                    0.1, 0.1, 0.1,
                                )),
                                ..default()
                            },
                            Name::new(TOWER_DEC),
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "-",
                                TextStyle {
                                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font: default(),
                                    font_size: 50.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(60.0),
                                    height: Val::Px(60.0),
                                    // border: UiRect::all(Val::Px(5.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                // border_color: BorderColor(Color::BLACK),
                                background_color: bevy::prelude::BackgroundColor(Color::rgb(
                                    0.1, 0.1, 0.1,
                                )),
                                ..default()
                            },
                            Name::new(DISK_INC),
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "+",
                                TextStyle {
                                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font: default(),
                                    font_size: 50.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(60.0),
                                    height: Val::Px(60.0),
                                    // border: UiRect::all(Val::Px(5.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                // border_color: BorderColor(Color::BLACK),
                                background_color: bevy::prelude::BackgroundColor(Color::rgb(
                                    0.1, 0.1, 0.1,
                                )),
                                ..default()
                            },
                            Name::new(DISK_DEC),
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "-",
                                TextStyle {
                                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font: default(),
                                    font_size: 50.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(60.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: bevy::prelude::BackgroundColor(Color::rgb(
                                    0.1, 0.1, 0.1,
                                )),
                                ..default()
                            },
                            Name::new(SPEED_INC),
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "+",
                                TextStyle {
                                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font: default(),
                                    font_size: 50.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(60.0),
                                    height: Val::Px(60.0),
                                    // border: UiRect::all(Val::Px(5.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                // border_color: BorderColor(Color::BLACK),
                                background_color: bevy::prelude::BackgroundColor(Color::rgb(
                                    0.1, 0.1, 0.1,
                                )),
                                ..default()
                            },
                            Name::new(SPEED_DEC),
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "-",
                                TextStyle {
                                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font: default(),
                                    font_size: 50.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(120.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: bevy::prelude::BackgroundColor(Color::rgb(
                                    0.1, 0.1, 0.1,
                                )),
                                ..default()
                            },
                            Name::new(START_STOP),
                        ))
                        .with_children(|parent| {
                            parent.spawn((TextBundle::from_section(
                                "start",
                                TextStyle {
                                    font: default(),
                                    font_size: 30.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ),));
                        });
                });
        });

    commands.spawn((Disks, PbrBundle { ..default() }));
}

///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children, &Name),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
    mut text_query: Query<&mut Text>,
    mut tower_state: ResMut<TowerConfig>,
    tower_query: Query<(Entity, &Tower)>,
    mut disks_query: Query<(Entity, &Disks)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    for (interaction, mut color, children, name) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.0, 0.0, 0.0).into();

                let (entity, _) = disks_query.iter_mut().next().unwrap();

                ///////////////////////////////////////////////////////////////

                if name.as_str() == START_STOP {
                    if text.sections[0].value == "start" {
                        text.sections[0].value = "stop".to_owned();
                        tower_state.running = true;
                    } else if text.sections[0].value == "stop" {
                        text.sections[0].value = "start".to_owned();
                        tower_state.running = false;
                    }
                }

                if name.as_str() == TOWER_INC {
                    tower_state.number_of_tower += 1;
                }

                if name.as_str() == TOWER_DEC {
                    tower_state.number_of_tower -= 1;
                }

                if name.as_str() == DISK_INC {
                    tower_state.number_of_disks += 1;
                }

                if name.as_str() == DISK_DEC {
                    tower_state.number_of_disks -= 1;
                }

                if name.as_str() == SPEED_INC {
                    tower_state.speed += 0.5;
                    let speed = tower_state.speed;
                    tower_state
                        .timer
                        .set_duration(Duration::from_secs_f32(speed));
                    return;
                }

                if name.as_str() == SPEED_DEC {
                    tower_state.speed -= 0.5;
                    let speed = tower_state.speed;
                    tower_state
                        .timer
                        .set_duration(Duration::from_secs_f32(speed));
                    return;
                }

                ///////////////////////////////////////////////////////////////

                for (entity, _) in tower_query.iter() {
                    commands.entity(entity).despawn();
                }
                for i in 0..tower_state.number_of_tower {
                    commands.spawn((
                        Tower,
                        PbrBundle {
                            mesh: meshes.add(bevy::math::primitives::Cylinder {
                                radius: 0.3,
                                half_height: f32::from(tower_state.number_of_disks) * 0.3,
                                ..default()
                            }),
                            material: materials.add(StandardMaterial {
                                base_color: Color::GRAY,
                                ..default()
                            }),
                            transform: Transform::from_translation(Vec3 {
                                x: f32::from(i) * 3.0,
                                y: f32::from(tower_state.number_of_disks) * 0.6 * 0.5,
                                z: 0.0,
                            }),
                            ..default()
                        },
                    ));
                }

                ///////////////////////////////////////////////////////////////

                commands.entity(entity).despawn_recursive();

                println!();

                commands
                    .spawn((Disks, PbrBundle { ..default() }))
                    .with_children(|parent| {
                        for i in 0..tower_state.number_of_disks {
                            println!("{:?}", Name::new("disk".to_string() + &i.to_string()));
                            parent.spawn((
                                Disk,
                                Name::new("disk".to_string() + &i.to_string()),
                                PbrBundle {
                                    mesh: meshes.add(bevy::math::primitives::Torus {
                                        minor_radius: 0.3,
                                        major_radius: 0.6,
                                        ..default()
                                    }),
                                    material: materials.add(StandardMaterial {
                                        base_color: Color::rgb(
                                            1.0,
                                            lerp(
                                                0.0,
                                                1.0,
                                                f32::from(i)
                                                    / f32::from(tower_state.number_of_disks),
                                            ),
                                            lerp(
                                                0.0,
                                                1.0,
                                                f32::from(i)
                                                    / f32::from(tower_state.number_of_disks),
                                            ),
                                        ),
                                        ..default()
                                    }),
                                    transform: Transform::from_translation(Vec3 {
                                        x: 0.0,
                                        y: f32::from(tower_state.number_of_disks - i - 1) * 0.6,
                                        z: 0.0,
                                    }),
                                    ..default()
                                },
                            ));
                        }
                    });

                if tower_state.running {
                    tower_state.moves = helper(
                        tower_state.number_of_tower.try_into().unwrap(),
                        tower_state.number_of_disks.into(),
                    );
                }
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = Color::rgb(0.3, 0.3, 0.3).into();
                // border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *color = Color::rgb(0.2, 0.2, 0.2).into();
                // border_color.0 = Color::BLACK;
            }
        }
    }
}
///////////////////////////////////////////////////////////////////////////////

fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    (1.0 - t) * v0 + t * v1
}

///////////////////////////////////////////////////////////////////////////////

fn helper(i: usize, j: u32) -> Vec<(usize, usize, u32)> {
    println!("{} pegs with {} disks", i, j);
    let f: Vec<u32> = (0..j).rev().collect();
    let mut rods = vec![];
    for n in 0..i {
        rods.push(vec![]);
    }
    rods[0] = f.clone();
    let mut moves = vec![];
    hanoi_general_rec(
        j.try_into().unwrap(),
        0,
        &mut rods,
        0,
        i - 1,
        (1..i - 1).collect(),
        &mut moves,
    );
    println!("{:?}", moves);
    println!();
    moves
}

///////////////////////////////////////////////////////////////////////////////

pub fn update_disk(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children, &Name),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
    mut text_query: Query<&mut Text>,
    mut tower_state: ResMut<TowerConfig>,
    mut disk_query: Query<(Entity, &mut Disk, &Name, &mut Transform)>,
    mut disks_query: Query<(Entity, &Disks)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
    time: Res<Time>,
) {
    if tower_state.running && !tower_state.moves.is_empty() {
        println!("animating");

        let (from, to, disk) = tower_state.moves[0];

        let (entity, disk_ob, _, mut transform) = disk_query
            .iter_mut()
            .filter(|e| e.2 == &Name::new("disk".to_owned() + &disk.to_string()))
            .next()
            .unwrap();

        println!("moving {} from {} to {}", disk, from, to);
        println!("entity: {:?}, transform: {:?}", entity, transform);

        let progress = tower_state.timer.fraction();

        println!("{}% through step", progress);

        let to_x_pos = f32::from(u16::try_from(to).unwrap()) * 3.0;
        let from_x_pos = f32::from(u16::try_from(from).unwrap()) * 3.0;
        let disk_height =
            f32::from(tower_state.number_of_disks - u16::try_from(disk).unwrap() - 1) * 0.6;
        let tower_height = f32::from(tower_state.number_of_disks) * 0.6 + 0.45;

        if progress <= 0.33333 {
            transform.set(Box::new(Transform::from_translation(Vec3 {
                x: from_x_pos,
                y: lerp(disk_height, tower_height, progress / 0.3),
                z: 0.0,
            })));
        } else if progress <= 0.6666 {
            transform.set(Box::new(Transform::from_translation(Vec3 {
                x: lerp(from_x_pos, to_x_pos, progress / 0.6),
                y: tower_height,
                z: 0.0,
            })));
        } else {
            transform.set(Box::new(Transform::from_translation(Vec3 {
                x: to_x_pos,
                y: lerp(tower_height, disk_height, progress / 1.0),
                z: 0.0,
            })));
        }

        println!("entity: {:?}, transform: {:?}", entity, transform);
        println!();
    }

    if tower_state.timer.tick(time.delta()).just_finished()
        && tower_state.running
        && !tower_state.moves.is_empty()
    {
        println!("next");

        let (from, to, disk) = tower_state.moves.remove(0);

        // let (entity, disk_ob, _, mut transform) = disk_query
        //     .iter_mut()
        //     .filter(|e| e.2 == &Name::new("disk".to_owned() + &disk.to_string()))
        //     .next()
        //     .unwrap();

        // println!("moving {} from {} to {}", disk, from, to);
        // println!("entity: {:?}, transform: {:?}", entity, transform);

        // transform.set(Box::new(Transform::from_translation(Vec3 {
        //     x: f32::from(u16::try_from(to).unwrap()) * 3.0,
        //     y: f32::from(tower_state.number_of_disks - u16::try_from(disk).unwrap() - 1) * 0.6,
        //     z: 0.0,
        // })));

        // println!("entity: {:?}, transform: {:?}", entity, transform);
        // println!();
    }
}
