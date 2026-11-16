//! Notification and toast alert state manager.

use dioxus::prelude::*;
use my_kit_core::base::id::short_id;
use my_kit_core::base::time::now_unix_secs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoticeLevel {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notice {
    pub id: String,
    pub level: NoticeLevel,
    pub message: String,
    pub timestamp: u64,
}

#[derive(Clone, Copy)]
pub struct NoticeManager {
    notices: Signal<Vec<Notice>>,
}

impl NoticeManager {
    pub fn new(notices: Signal<Vec<Notice>>) -> Self {
        Self { notices }
    }

    pub fn push(&self, level: NoticeLevel, message: impl Into<String>) -> String {
        let id = short_id(8);
        let notice = Notice {
            id: id.clone(),
            level,
            message: message.into(),
            timestamp: now_unix_secs(),
        };
        let mut n = self.notices;
        n.write().push(notice);
        id
    }

    pub fn info(&self, message: impl Into<String>) -> String {
        self.push(NoticeLevel::Info, message)
    }

    pub fn success(&self, message: impl Into<String>) -> String {
        self.push(NoticeLevel::Success, message)
    }

    pub fn warning(&self, message: impl Into<String>) -> String {
        self.push(NoticeLevel::Warning, message)
    }

    pub fn error(&self, message: impl Into<String>) -> String {
        self.push(NoticeLevel::Error, message)
    }

    pub fn dismiss(&self, id: &str) {
        let mut n = self.notices;
        n.write().retain(|item| item.id != id);
    }

    pub fn clear(&self) {
        let mut n = self.notices;
        n.write().clear();
    }

    pub fn list(&self) -> Vec<Notice> {
        (self.notices)().clone()
    }
}

/// Dioxus hook to provide and manage notifications across components.
pub fn use_notices() -> NoticeManager {
    let notices = use_context_provider(|| Signal::new(Vec::<Notice>::new()));
    NoticeManager::new(notices)
}
