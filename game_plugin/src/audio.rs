use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};
use rand::Rng;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin)
            .add_event::<PlaySoundEffect>()
            .add_system_set(
                SystemSet::on_enter(GameState::Menu).with_system(start_background.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InLevel).with_system(play_sound_effects.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Lost).with_system(play_sound_effects.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Finished).with_system(play_sound_effects.system()),
            );
    }
}

pub enum PlaySoundEffect {
    Jump,
    Land,
    Loose,
    Fall,
    Won,
}

fn play_sound_effects(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut events: EventReader<PlaySoundEffect>,
) {
    for event in events.iter() {
        match event {
            PlaySoundEffect::Jump => match rand::thread_rng().gen_range(0..2) {
                0 => audio.play(audio_assets.jump_1.clone()),
                _ => audio.play(audio_assets.jump_2.clone()),
            },
            PlaySoundEffect::Land => {
                audio.play(audio_assets.land_1.clone());
            }
            PlaySoundEffect::Fall => {
                audio.play(audio_assets.fall.clone());
            }
            PlaySoundEffect::Won => {
                audio.play(audio_assets.won.clone());
            }
            PlaySoundEffect::Loose => match rand::thread_rng().gen_range(0..2) {
                0 => audio.play(audio_assets.lose_1.clone()),
                _ => audio.play(audio_assets.lose_2.clone()),
            },
        }
    }
}

fn start_background(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play_looped(audio_assets.background.clone());
}
