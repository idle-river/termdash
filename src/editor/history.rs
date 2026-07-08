use super::{commands::EditorCommand, model::EditorState, refresh::RefreshLevelEvent};
use crate::level::{load::CurrentLevel, model::Level, registry::Levels};
use bevy::prelude::*;

#[derive(Default)]
pub struct History {
    stack: Vec<Level>,
    position: usize,
}

const HISTORY_LIMIT: usize = 100;

impl History {
    pub fn reset(&mut self, level: &Level) {
        self.stack.clear();
        self.position = 0;
        self.push(level);
    }

    /// Step forward (redo) or backward (undo).
    pub fn step(
        &mut self,
        direction: isize,
        level: &mut Level,
    ) -> Result<&'static str, &'static str> {
        let target_pos = self.position as isize + direction;

        if target_pos < 0 {
            return Err("there is nothing to undo");
        }
        if target_pos >= self.stack.len() as isize {
            return Err("there is nothing to redo");
        }

        self.position = target_pos as usize;
        *level = self.stack[self.position].clone();

        Ok(if direction < 0 {
            "undid level edit"
        } else {
            "redid level edit"
        })
    }

    pub fn push(&mut self, level: &Level) {
        self.stack.truncate(self.position + 1);
        self.stack.push(level.clone());

        if self.stack.len() > HISTORY_LIMIT {
            self.stack.remove(0);
        }

        self.position = self.stack.len() - 1;
    }
}

pub fn handle_history(
    mut editor: ResMut<EditorState>,
    current_level: Res<CurrentLevel>,
    mut levels: ResMut<Levels>,
    mut commands: MessageReader<EditorCommand>,
    mut events: MessageWriter<RefreshLevelEvent>,
) {
    let level = current_level.get_from_mut(&mut levels);

    for command in commands.read() {
        let direction = match *command {
            EditorCommand::FinishPendingEdit => {
                finish_pending_edit(&mut editor, level, &mut events);
                continue;
            }
            EditorCommand::Undo => Some(-1),
            EditorCommand::Redo => Some(1),
            _ => None,
        };

        if let Some(dir) = direction {
            finish_pending_edit(&mut editor, level, &mut events);

            match editor.history.step(dir, level) {
                Ok(status) => {
                    editor.clamp_selection(level);
                    editor.dirty = true;
                    editor.status = status.to_string();
                    editor.refresh_pending = false;
                    events.write(RefreshLevelEvent);
                }
                Err(err) => {
                    editor.status = err.to_string();
                }
            }
        }
    }
}

fn finish_pending_edit(
    editor: &mut EditorState,
    level: &Level,
    events: &mut MessageWriter<RefreshLevelEvent>,
) {
    if editor.refresh_pending {
        editor.clamp_selection(level);
        editor.history.push(level);
        editor.refresh_pending = false;
        events.write(RefreshLevelEvent);
    }
}
