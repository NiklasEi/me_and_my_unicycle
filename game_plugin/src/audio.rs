use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin)
            .add_event::<PlaySoundEffect>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(play_sound_effects.system()),
            );
    }
}

pub enum PlaySoundEffect {
    Jump,
}

fn play_sound_effects(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut events: EventReader<PlaySoundEffect>,
) {
    for event in events.iter() {
        match event {
            PlaySoundEffect::Jump => audio.play(audio_assets.jump_2.clone()),
        }
    }
}
