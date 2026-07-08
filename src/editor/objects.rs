use super::{commands::EditorCommand, model::EditorState};
use crate::{
    level::{
        load::CurrentLevel,
        model::{Level, PartialLevelObject},
        registry::Levels,
    },
    player::components::Player,
};
use bevy::prelude::*;

const DEFAULT_PREFAB: &str = "spike";
const DUPLICATE_OFFSET: Vec2 = Vec2::new(32.0, 0.0);

pub fn apply_object_commands(
    mut editor: ResMut<EditorState>,
    current_level: Res<CurrentLevel>,
    mut levels: ResMut<Levels>,
    mut commands: MessageReader<EditorCommand>,
    player: Query<&'static Transform, With<Player>>,
) {
    let level = current_level.get_from_mut(&mut levels);
    let player_pos = |editor: &mut EditorState| -> Option<Vec2> {
        if let Ok(player) = player.single() {
            Some(player.translation.xy())
        } else {
            editor.status = "no player found".to_string();
            None
        }
    };

    for command in commands.read() {
        match *command {
            EditorCommand::Duplicate => duplicate_selected(&mut editor, level),
            EditorCommand::DeleteSelected => delete_selected(&mut editor, level),
            EditorCommand::AddObjectAtPlayer => {
                if let Some(position) = player_pos(&mut editor) {
                    add_object_at(&mut editor, level, position);
                }
            }
            EditorCommand::SelectNearestToPlayer => {
                if let Some(position) = player_pos(&mut editor) {
                    select_nearest(&mut editor, level, position);
                }
            }
            _ => {}
        }
    }
}

fn duplicate_selected(editor: &mut EditorState, level: &mut Level) {
    let Some(index) = editor.selected_object else {
        editor.status = "no object selected to duplicate".to_string();
        return;
    };

    let Some(mut object) = level.objects.get(index).cloned() else {
        editor.status = "selected object no longer exists".to_string();
        editor.selected_object = None;
        return;
    };

    object.position += DUPLICATE_OFFSET;
    editor.selected_object = Some(level.objects.len());
    level.objects.push(object);
    editor.mark_changed();
}

fn select_nearest(editor: &mut EditorState, level: &Level, position: Vec2) {
    editor.selected_object = level
        .objects
        .iter()
        .enumerate()
        .map(|(index, object)| (index, object.position.distance(position)))
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index);
}

fn add_object_at(editor: &mut EditorState, level: &mut Level, position: Vec2) {
    editor.selected_object = Some(level.objects.len());
    level.objects.push(PartialLevelObject {
        prefab: Some(DEFAULT_PREFAB.to_string()),
        position,
        ..default()
    });
    editor.mark_changed();
}

fn delete_selected(editor: &mut EditorState, level: &mut Level) {
    let Some(index) = editor.selected_object else {
        return;
    };

    if index < level.objects.len() {
        level.objects.remove(index);
        editor.mark_changed();
    }

    editor.selected_object = None;
}
