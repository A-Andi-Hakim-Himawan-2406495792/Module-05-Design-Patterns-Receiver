use bambangshop_receiver::REQWEST_CLIENT;
use crate::model::notification::Notification;
use crate::model::subscriber::SubscriberRequest;
use crate::repository::notification::NotificationRepository;

pub struct NotificationService;

impl NotificationService {
    async fn subscribe_request(product_type: String, subscriber: SubscriberRequest) {
        let _ = REQWEST_CLIENT
            .post(format!("{}/notification/subscribe/{}",
                bambangshop_receiver::APP_CONFIG.get_instance_root_url(),
                product_type
            ))
            .json(&subscriber)
            .send()
            .await;
    }

    pub fn subscribe(product_type: String, subscriber: SubscriberRequest) {
        let product_type_clone = product_type.clone();
        let subscriber_clone = subscriber.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(Self::subscribe_request(product_type_clone, subscriber_clone));
        });
    }

    async fn unsubscribe_request(product_type: String, subscriber: SubscriberRequest) {
            let _ = REQWEST_CLIENT
                .delete(format!("{}/notification/unsubscribe/{}?subscriber_url={}",
                    bambangshop_receiver::APP_CONFIG.get_instance_root_url(),
                    product_type,
                    subscriber.url
                ))
                .send()
                .await;
        }

        pub fn unsubscribe(product_type: String, subscriber: SubscriberRequest) {
            let product_type_clone = product_type.clone();
            let subscriber_clone = subscriber.clone();
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(Self::unsubscribe_request(product_type_clone, subscriber_clone));
            });
        }

        pub fn receive_notification(notification: Notification) -> Notification {
            NotificationRepository::add(notification)
        }

        pub fn list_messages() -> Vec<String> {
                NotificationRepository::list_all_as_string()
            }
}