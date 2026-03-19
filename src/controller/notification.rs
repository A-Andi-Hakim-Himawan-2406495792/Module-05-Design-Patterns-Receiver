use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;
use crate::model::subscriber::SubscriberRequest;
use crate::model::notification::Notification;
use crate::service::notification::NotificationService;