use super::commands::EditorCommand;
use crate::{require_message, state::AppState};
use bevy::prelude::*;

pub fn apply_tests(
    mut reader: MessageReader<EditorCommand>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
) {
    require_message!(reader, EditorCommand::Test);

    commands.write_message(EditorCommand::FinishPendingEdit);
    next_state.set(AppState::Playing);
}
