use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};
use itertools::Itertools;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid>()
            .add_systems(Startup, (initialize_grid, setup_camera))
            .add_systems(Startup, spawn_background.after(initialize_grid));
    }
}

#[derive(Resource, Default)]
pub struct Grid {
    win_size: (f32, f32),
    gap_size: (f32, f32),
    board_size: (i32, i32),
}

const BOARDSIZE: i32 = 8;

pub fn initialize_grid(window_query: Query<&Window, With<PrimaryWindow>>, mut commands: Commands) {
    let window = window_query.get_single().unwrap();
    let window_size = (window.width(), window.height());

    commands.insert_resource(Grid::new(window_size));
}

impl Grid {
    // Create a new TranslationGrid instance
    pub fn new((wx, wy): (f32, f32)) -> Self {
        Grid {
            win_size: (wx, wy),
            board_size: (BOARDSIZE, BOARDSIZE),
            gap_size: (wx / BOARDSIZE as f32, wy / BOARDSIZE as f32),
        }
    }

    pub fn get_translation(&self, x: i32, y: i32) -> (f32, f32) {
        (
            -self.win_size.0 / 2.0 + (x as f32 + 0.5) / self.board_size.0 as f32 * self.win_size.0,
            -self.win_size.1 / 2.0 + (y as f32 + 0.5) / self.board_size.1 as f32 * self.win_size.1,
        )
    }

    pub fn get_indexes(&self, (tx, ty): (f32, f32)) -> (i32, i32) {
        (
            ((tx + self.win_size.0 / 2.) / self.gap_size.0) as i32 ,
            ((ty + self.win_size.1 / 2.) / self.gap_size.1) as i32 ,
        )
    }

    pub fn gap_size(&self) -> (f32, f32) {
        self.gap_size
    }
}

#[derive(Clone, Copy, Component)]
pub struct Tile(pub i32, pub i32);

pub fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Res<Grid>,
) {
    let mat1 = materials.add(Color::rgb(172. / 255., 206. / 255., 94. / 255.));
    let mat2 = materials.add(Color::rgb(114. / 255., 183. / 255., 106. / 255.));

    let tile = Mesh2dHandle(meshes.add(Rectangle::new(board.gap_size().0, board.gap_size().1)));

    (0..board.board_size.0)
        .cartesian_product(0..board.board_size.1)
        .for_each(|(x, y)| {
            let (translation_x, translation_y) = board.get_translation(x, y);
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
                Tile(x, y)
            ));
        })
}

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    // Spawn camera
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
