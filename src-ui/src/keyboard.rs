//! Keyboard shortcuts handling
//!
//! Provides keyboard shortcut support for staging/unstaging operations

#![allow(dead_code)]

use iced::keyboard;
use iced::Event;

/// Keyboard shortcut actions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShortcutAction {
    StageFile,
    UnstageFile,
    StageAll,
    UnstageAll,
    Refresh,
    ToggleChangesPanel,
    // Stash operations
    StashSave,
    StashPop,
    StashDrop,
    StashList,
}

/// Keyboard shortcut definition
#[derive(Debug, Clone)]
pub struct KeyboardShortcut {
    pub modifiers: keyboard::Modifiers,
    pub key: keyboard::Key,
    pub action: ShortcutAction,
}

impl KeyboardShortcut {
    /// Check if this shortcut matches the given event
    pub fn matches(&self, event: &Event) -> bool {
        if let Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event {
            return *key == self.key && modifiers.contains(self.modifiers);
        }
        false
    }
}

/// Get all registered keyboard shortcuts
pub fn get_shortcuts() -> Vec<KeyboardShortcut> {
    use keyboard::{Key, Modifiers};

    vec![
        // Ctrl+S: Stage selected file
        KeyboardShortcut {
            modifiers: Modifiers::CTRL,
            key: Key::Character("s".into()),
            action: ShortcutAction::StageFile,
        },
        // Ctrl+U: Unstage selected file
        KeyboardShortcut {
            modifiers: Modifiers::CTRL,
            key: Key::Character("u".into()),
            action: ShortcutAction::UnstageFile,
        },
        // Ctrl+Shift+S: Stage all
        KeyboardShortcut {
            modifiers: Modifiers::CTRL | Modifiers::SHIFT,
            key: Key::Character("s".into()),
            action: ShortcutAction::StageAll,
        },
        // Ctrl+Shift+U: Unstage all
        KeyboardShortcut {
            modifiers: Modifiers::CTRL | Modifiers::SHIFT,
            key: Key::Character("u".into()),
            action: ShortcutAction::UnstageAll,
        },
        // Ctrl+R: Refresh
        KeyboardShortcut {
            modifiers: Modifiers::CTRL,
            key: Key::Character("r".into()),
            action: ShortcutAction::Refresh,
        },
        // Ctrl+Shift+Z: Save stash
        KeyboardShortcut {
            modifiers: Modifiers::CTRL | Modifiers::SHIFT,
            key: Key::Character("z".into()),
            action: ShortcutAction::StashSave,
        },
        // Ctrl+Z: Pop stash
        KeyboardShortcut {
            modifiers: Modifiers::CTRL,
            key: Key::Character("z".into()),
            action: ShortcutAction::StashPop,
        },
        // Ctrl+Alt+Z: Drop stash
        KeyboardShortcut {
            modifiers: Modifiers::CTRL | Modifiers::ALT,
            key: Key::Character("z".into()),
            action: ShortcutAction::StashDrop,
        },
    ]
}

/// Find the action for a keyboard event
pub fn find_action(event: &Event) -> Option<ShortcutAction> {
    for shortcut in get_shortcuts() {
        if shortcut.matches(event) {
            return Some(shortcut.action);
        }
    }
    None
}

/// Format a keyboard shortcut for display
pub fn format_shortcut(shortcut: &KeyboardShortcut) -> String {
    let mut parts = Vec::new();

    if shortcut.modifiers.contains(keyboard::Modifiers::CTRL) {
        parts.push("Ctrl".to_string());
    }
    if shortcut.modifiers.contains(keyboard::Modifiers::SHIFT) {
        parts.push("Shift".to_string());
    }
    if shortcut.modifiers.contains(keyboard::Modifiers::ALT) {
        parts.push("Alt".to_string());
    }

    if let keyboard::Key::Character(c) = &shortcut.key {
        parts.push(c.to_uppercase());
    } else {
        parts.push(format!("{:?}", shortcut.key));
    }

    parts.join("+")
}

/// Get the description for a shortcut action
pub fn action_description(action: ShortcutAction) -> &'static str {
    match action {
        ShortcutAction::StageFile => "暂存选中文件",
        ShortcutAction::UnstageFile => "取消暂存选中文件",
        ShortcutAction::StageAll => "暂存全部",
        ShortcutAction::UnstageAll => "取消暂存全部",
        ShortcutAction::Refresh => "刷新",
        ShortcutAction::ToggleChangesPanel => "切换变更面板",
        ShortcutAction::StashSave => "保存储藏",
        ShortcutAction::StashPop => "弹出储藏",
        ShortcutAction::StashDrop => "删除储藏",
        ShortcutAction::StashList => "列出储藏",
    }
}
