use bevy::prelude::*;

#[derive(Message, Clone, Copy)]
pub enum EditorCommand {
    Save,
    Test,
    FinishPendingEdit,
    Undo,
    Redo,
    Duplicate,
    SelectNearestToPlayer,
    AddObjectAtPlayer,
    DeleteSelected,
}
