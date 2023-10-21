#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod loading;
mod menu;
pub mod cell_image;

pub mod pipeline;




use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin, DiagnosticsStore};
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::{RenderApp, RenderSet};
use bevy::window::PrimaryWindow;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::cell_image::CellImage;
use crate::pipeline::PipelinesPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

const SIMULATION_SIZE: (u32, u32) = (1280, 720);
const WORKGROUP_SIZE: u32 = 8;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            ActionsPlugin,
            InternalAudioPlugin
        ))
            .add_systems(Startup, setup)
            .add_plugins((
                ExtractResourcePlugin::<CellImage>::default()))
            .add_plugins(PipelinesPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin::default(), LogDiagnosticsPlugin::default()))
                .add_systems(Update, display_fps);
        }
    }
}
fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>){
    let width: u32 = SIMULATION_SIZE.0;
    let height: u32 = SIMULATION_SIZE.1;
    let image = cell_image::create_image(width, height);
    let image :Handle<Image> = images.add(image);
    commands.spawn(SpriteBundle{
        sprite: Sprite{
            color: Default::default(),
            custom_size: Some(Vec2::new(width as f32, height as f32)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });
    commands.insert_resource(cell_image::CellImage(image))
}

fn display_fps( diagnostics: Res<DiagnosticsStore>, mut windows: Query<&mut Window, With<PrimaryWindow>>){
    if let Ok(mut window) = windows.get_single_mut(){
        if let Some(fps_raw) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS){
            if let Some(fps_smoothed) = fps_raw.smoothed(){
                window.title = format!("Falling Sand Game {fps_smoothed:.2} fps")
            }
        }
    }
}
