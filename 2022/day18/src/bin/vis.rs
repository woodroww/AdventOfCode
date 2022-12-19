use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use std::collections::HashSet;

use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;
use day18::{
    cardinal_3d, center, fill_outside, input_txt, max_xyz, parse_cubes, InputFile, Point3D,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Boiling Boulders".to_string(),
                width: 800.0,
                height: 800.0,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
        .add_startup_system(spawn_axis)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_cubes)
        .add_system(cube_click)
        .add_system(animate_fill)
        .add_system(pan_orbit_camera)
        .run();
}

#[derive(Resource)]
pub struct BoulderAssets {
    cubes: HashSet<Point3D>,
    empties: HashSet<Point3D>,
    fill_stack: Vec<Point3D>,
    max_x: usize,
    max_y: usize,
    max_z: usize,
}

#[derive(Component)]
pub struct BoulderEmpty {
    location: Point3D,
}

#[derive(Component)]
pub struct BoulderCube {
    location: Point3D,
}

fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    let input = input_txt(InputFile::Example);
    let cubes = parse_cubes(&input);

    let cubes = cubes
        .into_iter()
        .map(|mut cube| {
            cube.x += 1;
            cube.y += 1;
            cube.z += 1;
            cube
        })
        .collect();

    let (max_x, max_y, max_z) = max_xyz(&cubes);
    /*println!("maxs {} {} {}", max_x, max_y, max_z);
    let empties = fill_outside(
        &cubes,
        Point3D { x: 0, y: 0, z: 0 },
        max_x,
        max_y,
        max_z,
    );*/
    let mut fill_stack = Vec::new();
    fill_stack.push(Point3D { x: 0, y: 0, z: 0 });
    let empties = HashSet::new();
    commands.insert_resource(BoulderAssets {
        cubes,
        empties,
        max_x: max_x + 2,
        max_y: max_y + 2,
        max_z: max_z + 2,
        fill_stack,
    });
}

pub fn animate_fill(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut assets: ResMut<BoulderAssets>,
) {
    if assets.fill_stack.len() > 0 {
        let p = assets.fill_stack.pop().unwrap();
        if assets.cubes.contains(&p) == false {
            let did_insert = assets.empties.insert(p);
            if did_insert {
                let transform = Transform::from_xyz(p.x as f32, p.y as f32, p.z as f32);
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::rgba(0.3, 0.3, 0.9, 0.2).into()),
                        transform,
                        ..default()
                    })
                    .insert(NotShadowCaster)
                    //.insert(PickableBundle::default())
                    .insert(BoulderCube { location: p });
                let neighbors = cardinal_3d(&p, assets.max_x, assets.max_y, assets.max_z);
                assets.fill_stack.extend_from_slice(&neighbors);
            }
        }
    }
}

fn spawn_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<BoulderAssets>,
) {
    for cube in assets.cubes.iter() {
        let transform = Transform::from_xyz(cube.x as f32, cube.y as f32, cube.z as f32);
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.9, 0.3, 0.3).into()),
                transform,
                ..default()
            })
            .insert(PickableBundle::default())
            .insert(BoulderCube { location: *cube });
    }
}

fn cube_click(selection: Query<(&BoulderCube, &Selection)>) {
    for (cube, selection) in &selection {
        if selection.selected() {
            println!("click boy {:?}", cube.location);
        }
    }
}

fn spawn_axis(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let length = 20.0;
    let width = 0.25;
    //let x = Box::new(x_length, y_length, z_length);
    let x = shape::Box::new(length, width, width);
    let y = shape::Box::new(width, length, width);
    let z = shape::Box::new(width, width, length);

    let mut transform = Transform::default();
    transform.translation.x = length / 2.0;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(x)),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform,
            ..default()
        },
        NotShadowCaster,
    ));
    let mut transform = Transform::default();
    transform.translation.y = length / 2.0;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(y)),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform,
            ..default()
        },
        NotShadowCaster,
    ));
    let mut transform = Transform::default();
    transform.translation.z = length / 2.0;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(z)),
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            transform,
            ..default()
        },
        NotShadowCaster,
    ));
}

// -----------------------------------------------------------------------------
// Camera things
// -----------------------------------------------------------------------------

fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<BoulderAssets>,
) {
    let c = center(&assets.cubes);
    let center_point = Vec3 {
        x: c.x as f32,
        y: c.y as f32,
        z: c.z as f32,
    };
    let mut transform = Transform::default();
    transform.translation = center_point;
    transform.translation.z = 50.0;

    let cam = Camera3dBundle {
        transform,
        ..Default::default()
    };

    commands.spawn((
        cam,
        PickingCameraBundle::default(),
        PanOrbitCamera {
            radius: (transform.translation - center_point).length(),
            focus: center_point,
            ..Default::default()
        },
    ));

    let p = 20.0;
    let n = -5.0;
    let intensity = 2500.0;

    let lights = vec![
        Transform::from_xyz(n, n, n),
        Transform::from_xyz(n, n, p),
        Transform::from_xyz(n, p, p),
        Transform::from_xyz(p, p, p),
        Transform::from_xyz(p, p, n),
        Transform::from_xyz(p, n, n),
        Transform::from_xyz(p, n, p),
        Transform::from_xyz(n, p, n),
    ];

    for light in lights.into_iter() {
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity,
                shadows_enabled: true,
                ..default()
            },
            transform: light,
            ..default()
        });
    }
}

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}
/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
fn pan_orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.002;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}
