use super::{commands::EditorCommand, model::EditorState};
use crate::{
    level::{load::CurrentLevel, registry::Levels},
    require_message,
};
use bevy::prelude::*;

pub fn apply_saves(
    mut editor: ResMut<EditorState>,
    current_level: Res<CurrentLevel>,
    mut levels: ResMut<Levels>,
    mut reader: MessageReader<EditorCommand>,
    mut commands: Commands,
) {
    require_message!(reader, EditorCommand::Save);

    commands.write_message(EditorCommand::FinishPendingEdit);
    let level = current_level.get_from(&levels).clone();
    let path = levels.save(level, current_level.index());

    match path {
        Ok(path) => {
            editor.dirty = false;
            editor.status = format!("saved {}", path.display());
        }
        Err(err) => editor.status = format!("save failed: {}", err),
    }
}
