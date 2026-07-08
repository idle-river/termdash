use super::commands::EditorCommand;
use crate::input::{EguiInput, InputState};
use bevy::prelude::*;
use bevy_egui::egui;

macro_rules! shortcut {
    ($key:ident => $command:expr) => {
        (
            EguiInput::with(egui::Key::$key, egui::Modifiers::CTRL),
            $command,
        )
    };
}

macro_rules! key {
    ($key:ident => $command:expr) => {
        (EguiInput::new(egui::Key::$key), $command)
    };
}

pub fn editor_input(input: Res<InputState>, mut commands: MessageWriter<EditorCommand>) {
    for (key, command) in [
        shortcut!(S => EditorCommand::Save),
        shortcut!(Z => EditorCommand::Undo),
        shortcut!(Y => EditorCommand::Redo),
        shortcut!(D => EditorCommand::Duplicate),
        key!(Delete => EditorCommand::DeleteSelected),
        key!(F => EditorCommand::SelectNearestToPlayer),
    ] {
        if input.egui_pressed(key) {
            commands.write(command);
        }
    }
}
