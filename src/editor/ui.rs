use super::{
    commands::EditorCommand,
    model::{EditorCamera, EditorState},
};
use crate::level::{load::CurrentLevel, registry::Levels};
use bevy::{ecs::reflect::AppTypeRegistry, ecs::system::SystemParam, prelude::*};
use bevy_egui::{EguiContext, egui};
use bevy_inspector_egui::reflect_inspector;

pub fn show_editor(
    mut egui_context: Single<&mut EguiContext, With<EditorCamera>>,
    mut params: EditorUiParams,
) {
    let ctx = egui_context.get_mut();
    let mut changed = false;
    {
        let level = params.current_level.get_from_mut(&mut params.levels);

        egui::TopBottomPanel::top("editor_toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for (label, cmd) in [("Save", EditorCommand::Save), ("Test", EditorCommand::Test)] {
                    if ui.button(label).clicked() {
                        params.commands.write(cmd);
                    }
                }

                let marker = if params.editor.dirty { "*" } else { "" };
                ui.label(format!("{marker}{}", params.editor.status));
            });
        });

        egui::SidePanel::left("object_selection")
            .resizable(true)
            .default_width(260.0)
            .show(ctx, |ui| {
                ui.heading("Objects");

                for (label, cmd) in [
                    (
                        "Select nearest to player",
                        EditorCommand::SelectNearestToPlayer,
                    ),
                    ("Add object at player", EditorCommand::AddObjectAtPlayer),
                    ("Duplicate selected", EditorCommand::Duplicate),
                    ("Delete selected", EditorCommand::DeleteSelected),
                ] {
                    if ui.button(label).clicked() {
                        params.commands.write(cmd);
                    }
                }

                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (index, object) in level.objects.iter().enumerate() {
                        let prefab = object.prefab.as_deref().unwrap_or("custom");
                        let selected = params.editor.selected_object == Some(index);
                        let label = format!(
                            "#{index} {prefab} ({:.0}, {:.0})",
                            object.position.x, object.position.y
                        );

                        if ui.selectable_label(selected, label).clicked() {
                            params.editor.selected_object = Some(index);
                        }
                    }
                });
            });

        egui::SidePanel::right("selected_object")
            .resizable(true)
            .default_width(320.0)
            .show(ctx, |ui| {
                ui.heading("Object Inspector");

                let Some(index) = params.editor.selected_object else {
                    ui.label("No object selected.");
                    return;
                };

                let Some(object) = level.objects.get_mut(index) else {
                    ui.label("Selected object no longer exists.");
                    return;
                };

                let registry = params.type_registry.read();
                changed |= reflect_inspector::ui_for_value(object, ui, &registry);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Level Data");
            egui::ScrollArea::both().show(ui, |ui| {
                let registry = params.type_registry.read();
                changed |= reflect_inspector::ui_for_value(level, ui, &registry);
            });
        });

        if changed {
            params.editor.clamp_selection(level);
            params.editor.mark_changed();
        }

        let editing_interactively =
            ctx.wants_keyboard_input() || ctx.input(|input| input.pointer.any_down());

        if !editing_interactively {
            params.commands.write(EditorCommand::FinishPendingEdit);
        }
    }
}

#[derive(SystemParam)]
pub struct EditorUiParams<'w> {
    editor: ResMut<'w, EditorState>,
    current_level: Res<'w, CurrentLevel>,
    levels: ResMut<'w, Levels>,
    type_registry: Res<'w, AppTypeRegistry>,
    commands: MessageWriter<'w, EditorCommand>,
}
