pub mod commands;
pub mod history;
pub mod input;
pub mod model;
pub mod objects;
pub mod refresh;
pub mod save;
pub mod testing;
pub mod ui;
pub mod window;

use crate::editor::model::EditorWindow;
use crate::level::registry::Levels;
use crate::{editor::commands::EditorCommand, level::load::CurrentLevel};
use avian2d::prelude::ColliderConstructor;
use bevy::prelude::*;
use bevy_egui::{EguiGlobalSettings, EguiPlugin};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use model::{EditorState, EditorWindowPass};

use crate::level::model::register_level_data_types;
use crate::state::AppState;
use bevy::window::{WindowFocused, WindowMoved};

const FOCUS_TEST_GRACE_SECS: f32 = 1.0;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin::default(), DefaultInspectorConfigPlugin))
            .init_resource::<EditorState>()
            .add_message::<commands::EditorCommand>()
            .add_message::<refresh::RefreshLevelEvent>()
            .register_type::<ColliderConstructor>()
            .add_systems(Startup, |mut settings: ResMut<EguiGlobalSettings>| {
                settings.auto_create_primary_context = false;
            })
            .add_systems(OnEnter(AppState::Editing), window::open_editor_window)
            .add_systems(OnEnter(AppState::MainMenu), window::close_editor_window)
            .add_systems(
                Update,
                (window::handle_focus_change, window::handle_window_close),
            )
            .add_systems(Update, refresh::refresh_level)
            .add_systems(
                EditorWindowPass,
                (
                    sync_to_current_level,
                    input::editor_input,
                    handle_focus,
                    ui::show_editor,
                    history::handle_history,
                    objects::apply_object_commands,
                    save::apply_saves,
                    testing::apply_tests,
                )
                    .chain()
                    .run_if(in_state(AppState::Editing)),
            );

        register_level_data_types(app);
    }
}

pub fn sync_to_current_level(
    mut editor: ResMut<EditorState>,
    current_level: Res<CurrentLevel>,
    levels: Res<Levels>,
) {
    if current_level.is_changed() {
        let level = current_level.get_from(&levels);
        editor.history.reset(level);
        editor.dirty = false;
        editor.refresh_pending = false;
        editor.clamp_selection(level);
    }
}

pub fn handle_focus(
    windows: Query<Entity, With<EditorWindow>>,
    mut events: (MessageReader<WindowFocused>, MessageReader<WindowMoved>),
    time: Res<Time>,
    mut editor: ResMut<EditorState>,
    mut commands: MessageWriter<EditorCommand>,
) {
    let (ref mut window_focus, ref mut window_moved) = events;

    if window_moved
        .read()
        .any(|event| windows.contains(event.window))
    {
        editor.focus_test_timer = None;
        return;
    }

    match window_focus
        .read()
        .filter(|event| windows.contains(event.window))
        .last()
        .map(|event| event.focused)
    {
        Some(false) => {
            editor.focus_test_timer =
                Some(Timer::from_seconds(FOCUS_TEST_GRACE_SECS, TimerMode::Once));
        }
        Some(true) => editor.focus_test_timer = None,
        None => {}
    }

    let Some(timer) = editor.focus_test_timer.as_mut() else {
        return;
    };

    timer.tick(time.delta());
    if timer.is_finished() {
        editor.focus_test_timer = None;
        commands.write(EditorCommand::Test);
    }
}
