use bevy::prelude::*;
use bevy_egui::{EguiContext, egui};
use bevy_ratatui::event::KeyMessage;
use ratatui::crossterm::event::{KeyCode as TerminalKeyCode, KeyEventKind};
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct EguiInput {
    pub key: egui::Key,
    pub modifiers: egui::Modifiers,
}

impl EguiInput {
    pub const fn new(key: egui::Key) -> Self {
        Self {
            key,
            modifiers: egui::Modifiers::NONE,
        }
    }

    pub fn with(key: egui::Key, modifiers: egui::Modifiers) -> Self {
        Self {
            key,
            modifiers: normalize_egui_modifiers(modifiers),
        }
    }
}

#[derive(Resource, Default)]
pub struct InputState {
    terminal: HashSet<TerminalKeyCode>,
    egui: HashSet<EguiInput>,
}

impl InputState {
    pub fn pressed(&self, key: TerminalKeyCode) -> bool {
        self.terminal.contains(&key)
    }

    pub fn egui_pressed(&self, input: EguiInput) -> bool {
        self.egui.contains(&input)
    }

    fn clear(&mut self) {
        self.terminal.clear();
        self.egui.clear();
    }
}

fn normalize_egui_modifiers(modifiers: egui::Modifiers) -> egui::Modifiers {
    egui::Modifiers {
        alt: modifiers.alt,
        ctrl: modifiers.ctrl,
        shift: modifiers.shift,
        mac_cmd: modifiers.mac_cmd,
        command: false,
    }
}

fn update_terminal_input(key_messages: &mut MessageReader<KeyMessage>, input: &mut InputState) {
    for key in key_messages
        .read()
        .filter(|key| key.kind == KeyEventKind::Press)
    {
        input.terminal.insert(key.code);
    }
}

fn update_egui_input(egui_context: &mut EguiContext, input: &mut InputState) {
    egui_context.get_mut().input(|ctx| {
        for event in &ctx.events {
            let egui::Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } = event
            else {
                continue;
            };

            input.egui.insert(EguiInput {
                key: *key,
                modifiers: normalize_egui_modifiers(*modifiers),
            });
        }
    });
}

fn update(
    mut key_messages: MessageReader<KeyMessage>,
    mut input: ResMut<InputState>,
    mut egui_context: Query<&mut EguiContext>,
) {
    input.clear();

    update_terminal_input(&mut key_messages, &mut input);

    let Ok(mut egui_context) = egui_context.single_mut() else {
        return;
    };

    update_egui_input(&mut egui_context, &mut input);
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .add_systems(PreUpdate, update);
    }
}
