use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        AssetLoader::new(GameState::Loading, GameState::Menu)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .build(app);
    }
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/jump_1.ogg")]
    pub jump_1: Handle<AudioSource>,
    #[asset(path = "audio/jump_2.ogg")]
    pub jump_2: Handle<AudioSource>,
    #[asset(path = "audio/land_1.ogg")]
    pub land_1: Handle<AudioSource>,
    #[asset(path = "audio/loose_1.ogg")]
    pub loose_1: Handle<AudioSource>,
    #[asset(path = "audio/loose_2.ogg")]
    pub loose_2: Handle<AudioSource>,
    #[asset(path = "audio/won.ogg")]
    pub won: Handle<AudioSource>,
    #[asset(path = "audio/fall.ogg")]
    pub fall: Handle<AudioSource>,
    #[asset(path = "audio/background.ogg")]
    pub background: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/wheel.png")]
    pub wheel: Handle<Texture>,
    #[asset(path = "textures/head.png")]
    pub head: Handle<Texture>,
    #[asset(path = "textures/body.png")]
    pub body: Handle<Texture>,
    #[asset(path = "textures/background_1.png")]
    pub background_1: Handle<Texture>,
    #[asset(path = "textures/background_2.png")]
    pub background_2: Handle<Texture>,
    #[asset(path = "textures/background_3.png")]
    pub background_3: Handle<Texture>,
    #[asset(path = "textures/tutorial.png")]
    pub tutorial: Handle<Texture>,
    #[asset(path = "textures/tutorial_jump.png")]
    pub tutorial_jump: Handle<Texture>,
    #[asset(path = "textures/tutorial_falling.png")]
    pub tutorial_falling: Handle<Texture>,
    #[asset(path = "textures/tutorial_restart.png")]
    pub tutorial_restart: Handle<Texture>,
    #[asset(path = "textures/finish.png")]
    pub finish: Handle<Texture>,
}
