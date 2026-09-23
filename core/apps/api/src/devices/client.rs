use crate::admin::model::AdminDevice;
use api_connector::PusherClient;
use primitives::Device;
use push_notification::{GorushNotification, PushNotification, PushNotificationTypes};
use std::error::Error;
use storage::{Database, DatabaseError, DevicesRepository, PriceAlertsRepository, models::UpdateDeviceRow};

use super::clients::WalletsClient;

#[derive(Clone)]
pub struct DevicesClient {
    database: Database,
    pusher: PusherClient,
}

impl DevicesClient {
    pub fn new(database: Database, pusher: PusherClient) -> Self {
        Self { database, pusher }
    }

    pub async fn add_device(&self, device: Device) -> Result<Device, Box<dyn Error + Send + Sync>> {
        let add_device = UpdateDeviceRow::from_primitive(device);
        Ok(self.database.run(move |client| client.add_device(add_device)).await?)
    }

    pub async fn get_device(&self, device_id: &str) -> Result<Device, Box<dyn Error + Send + Sync>> {
        let device_id = device_id.to_string();
        Ok(self.database.run(move |client| client.get_device(&device_id)).await?)
    }

    pub async fn get_admin_device(&self, device_id: &str, wallets: &WalletsClient) -> Result<AdminDevice, Box<dyn Error + Send + Sync>> {
        let device_id = device_id.to_string();
        let (device, price_alert_count) = self
            .database
            .run(move |client| -> Result<_, DatabaseError> {
                let device = client.get_device_row(&device_id)?;
                let price_alert_count = client.count_price_alerts_for_device_id(device.id)?;
                Ok((device, price_alert_count))
            })
            .await?;
        Ok(AdminDevice {
            price_alert_count,
            wallets: wallets.get_wallet_overviews(device.id).await?,
            device: device.as_primitive(),
        })
    }

    pub async fn update_device(&self, device: Device) -> Result<Device, Box<dyn Error + Send + Sync>> {
        let update_device = UpdateDeviceRow::from_primitive(device);
        Ok(self.database.run(move |client| client.update_device(update_device)).await?)
    }

    pub async fn send_push_notification_device(&self, device_id: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let device = self.get_device(device_id).await?;
        let notifications: Vec<_> = GorushNotification::from_device(
            device,
            "Test Notification".to_string(),
            "Test Message".to_string(),
            PushNotification {
                notification_type: PushNotificationTypes::Test,
                data: None,
            },
        )
        .into_iter()
        .collect();
        Ok(self.pusher.push_notifications(notifications).await?.response.counts > 0)
    }

    pub async fn is_device_registered(&self, device_id: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let device_id = device_id.to_string();
        Ok(self.database.run(move |client| client.get_device_exist(&device_id)).await?)
    }
}
