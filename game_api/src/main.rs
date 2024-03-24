use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::{PrimaryWindow},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (initialize_grid, setup_camera))
        .add_systems(Startup, spawn_background.after(initialize_grid))
        .run();
}

#[derive(Resource, Default)]
pub struct Board {
    pub translation_grid: Vec<Vec<(f32, f32)>>,
    gap_size: f32,
}

const BOARDSIZE: i32 = 8;

pub fn initialize_grid(window_query: Query<&Window, With<PrimaryWindow>>, mut commands: Commands) {
    let window = window_query.get_single().unwrap();
    let min_window_size = f32::min(window.width(), window.height());

    commands.insert_resource(Board::new(min_window_size));
}

impl Board {
    // Create a new TranslationGrid instance
    pub fn new(window_size: f32) -> Self {
        let mut grid = Vec::new();
        for x in 0..BOARDSIZE {
            let mut row = Vec::new();
            for y in 0..BOARDSIZE {
                let translation_x =
                    -window_size / 2.0 + (x as f32 + 0.5) / BOARDSIZE as f32 * window_size;
                let translation_y =
                    -window_size / 2.0 + (y as f32 + 0.5) / BOARDSIZE as f32 * window_size;
                row.push((translation_x, translation_y));
            }
            grid.push(row);
        }
        Board {
            translation_grid: grid,
            gap_size: window_size / BOARDSIZE as f32,
        }
    }

    pub fn gap_size(&self) -> f32 {
        self.gap_size
    }
}

#[derive(Component)]
pub struct Tile {}


pub fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Res<Board>,
) {
    let mat1 = materials.add(Color::rgb(172. / 255., 206. / 255., 94. / 255.));
    let mat2 = materials.add(Color::rgb(114. / 255., 183. / 255., 106. / 255.));

    let tile = Mesh2dHandle(meshes.add(Rectangle::new(board.gap_size(), board.gap_size())));

    board
        .translation_grid
        .iter()
        .enumerate()
        .for_each(|(y, row)| {
            row.iter()
                .enumerate()
                .for_each(|(x, &(translation_x, translation_y))| {
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: tile.clone(),
                            material: match (x + y) % 2 {
                                0 => mat1.clone(),
                                1 => mat2.clone(),
                                _ => unreachable!(),
                            },
                            transform: Transform::from_xyz(
                                translation_x,
                                translation_y,
                                0.0, // Z translation, assuming 2D
                            ),
                            ..Default::default()
                        },
                        Tile {},
                    ));
                });
        });
}


fn setup_camera(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());
}
