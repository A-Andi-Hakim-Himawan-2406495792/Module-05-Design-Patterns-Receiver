use lazy_static::lazy_static;
use std::sync::RwLock;
use crate::model::notification::Notification;

lazy_static! {
    static ref NOTIFICATIONS: RwLock<Vec<Notification>> = RwLock::new(Vec::new());
}

pub struct NotificationRepository;

impl NotificationRepository {
    pub fn add(notification: Notification) -> Notification {
        let mut notifications = NOTIFICATIONS.write().unwrap();
        notifications.push(notification.clone());
        notification
    }

    pub fn list_all_as_string() -> Vec<String> {
        let notifications = NOTIFICATIONS.read().unwrap();
        notifications.iter().map(|n| n.to_string()).collect()
    }
}