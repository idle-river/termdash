use crate::editor::history::History;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;

use crate::level::model::Level;

#[derive(Component)]
pub struct EditorWindow;

#[derive(Component)]
pub struct EditorCamera;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EditorWindowPass;

#[derive(Default, Resource)]
pub struct EditorState {
    pub selected_object: Option<usize>,
    pub status: String,
    pub dirty: bool,
    pub refresh_pending: bool,
    pub history: History,
    pub focus_test_timer: Option<Timer>,
}

impl EditorState {
    pub fn mark_changed(&mut self) {
        self.dirty = true;
        self.status = "edited level data".to_string();
        self.refresh_pending = true;
    }

    pub fn clamp_selection(&mut self, level: &Level) {
        if self
            .selected_object
            .is_some_and(|index| index >= level.objects.len())
        {
            self.selected_object = None;
        }
    }
}
