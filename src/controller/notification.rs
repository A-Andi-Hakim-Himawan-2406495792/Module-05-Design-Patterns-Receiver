use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;
use crate::model::subscriber::SubscriberRequest;
use crate::model::notification::Notification;
use crate::service::notification::NotificationService;

#[post("/subscribe/<product_type>", data = "<subscriber>")]
pub fn subscribe(product_type: String, subscriber: Json<SubscriberRequest>)
    -> status::Custom<Json<SubscriberRequest>>
{
    NotificationService::subscribe(product_type, subscriber.clone().into_inner());
    status::Custom(Status::Ok, subscriber)
}