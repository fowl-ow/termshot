use bevy::{
    app::Plugin,
    ecs::resource::Resource,
    state::{app::AppExtStates, state::States},
};

pub struct TermshotGameStatePlugin;

impl Plugin for TermshotGameStatePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<GameState>();
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Intro,
    Ingame,
    Exiting,
}
