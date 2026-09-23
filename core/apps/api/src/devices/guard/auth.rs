use rocket::Request;
use rocket::http::Status;
use rocket::outcome::Outcome::{Error, Success};
use rocket::request::Outcome;
use storage::{Database, DatabaseError, DeviceRecord, DevicesRepository, WalletRecord, WalletsRepository};

use crate::devices::constants::DEVICE_ID_LENGTH;
use crate::devices::error::DeviceError;
use crate::devices::signature::verify_request_auth;
use crate::responders::cache_error;

pub(super) struct AuthResult {
    pub(super) device_id: String,
    pub(super) wallet_id: Option<String>,
}

pub(super) fn auth_error_outcome<T>(req: &Request<'_>, error: DeviceError, device_id: Option<&str>, wallet_id: Option<&str>) -> Outcome<T, String> {
    let status = match error {
        DeviceError::MissingHeader(_)
        | DeviceError::InvalidDeviceId
        | DeviceError::InvalidTimestamp
        | DeviceError::TimestampExpired
        | DeviceError::InvalidSignature
        | DeviceError::MissingWalletId
        | DeviceError::InvalidAuthorizationFormat => Status::Unauthorized,
        DeviceError::DeviceNotFound | DeviceError::WalletNotFound => Status::NotFound,
        DeviceError::DatabaseUnavailable | DeviceError::DatabaseError => Status::InternalServerError,
    };
    let message = format_auth_error_message(&error, device_id, wallet_id);
    cache_error(req, &message);
    Error((status, message))
}

fn format_auth_error_message(error: &DeviceError, device_id: Option<&str>, wallet_id: Option<&str>) -> String {
    let mut message = error.to_string();
    if let Some(id) = device_id {
        message.push_str(&format!(" device_id={id}"));
    }
    if let Some(id) = wallet_id {
        message.push_str(&format!(" wallet_id={id}"));
    }
    message
}

pub(super) async fn authenticate<T>(req: &Request<'_>) -> Result<AuthResult, Outcome<T, String>> {
    let auth = match verify_request_auth(req).await {
        Ok(auth) => auth,
        Err((status, message)) => {
            cache_error(req, &message);
            return Err(Error((status, message)));
        }
    };

    if auth.device_id.len() != DEVICE_ID_LENGTH {
        return Err(auth_error_outcome(req, DeviceError::InvalidDeviceId, Some(&auth.device_id), None));
    }

    Ok(AuthResult {
        device_id: auth.device_id.clone(),
        wallet_id: auth.wallet_id.clone(),
    })
}

pub(super) async fn lookup_device<T>(req: &Request<'_>, device_id: &str) -> Result<DeviceRecord, Outcome<T, String>> {
    let Success(database) = req.guard::<&rocket::State<Database>>().await else {
        return Err(auth_error_outcome(req, DeviceError::DatabaseUnavailable, Some(device_id), None));
    };

    let lookup_id = device_id.to_string();
    match database.run(move |client| Ok::<_, DatabaseError>(client.get_device_record(&lookup_id))).await {
        Ok(Ok(device)) => Ok(device),
        Ok(Err(_)) => Err(auth_error_outcome(req, DeviceError::DeviceNotFound, Some(device_id), None)),
        Err(_) => Err(auth_error_outcome(req, DeviceError::DatabaseError, Some(device_id), None)),
    }
}

pub(super) async fn lookup_device_wallet<T>(req: &Request<'_>, device_id: &str, wallet_id: &str) -> Result<(DeviceRecord, WalletRecord), Outcome<T, String>> {
    let Success(database) = req.guard::<&rocket::State<Database>>().await else {
        return Err(auth_error_outcome(req, DeviceError::DatabaseUnavailable, Some(device_id), None));
    };

    let lookup_device_id = device_id.to_string();
    let lookup_wallet_id = wallet_id.to_string();
    let lookup = database
        .run(move |client| -> Result<_, DatabaseError> {
            let Ok(device) = client.get_device_record(&lookup_device_id) else {
                return Ok(Err(DeviceError::DeviceNotFound));
            };
            Ok(match client.get_wallet_by_device_and_identifier(device.id, &lookup_wallet_id) {
                Ok(wallet) => Ok((device, wallet)),
                Err(error) if error.is_not_found() => Err(DeviceError::WalletNotFound),
                Err(_) => Err(DeviceError::DatabaseError),
            })
        })
        .await;

    match lookup {
        Ok(Ok(records)) => Ok(records),
        Ok(Err(DeviceError::DeviceNotFound)) => Err(auth_error_outcome(req, DeviceError::DeviceNotFound, Some(device_id), None)),
        Ok(Err(error)) => Err(auth_error_outcome(req, error, Some(device_id), Some(wallet_id))),
        Err(_) => Err(auth_error_outcome(req, DeviceError::DatabaseError, Some(device_id), None)),
    }
}

#[cfg(test)]
mod tests {
    use super::format_auth_error_message;
    use crate::devices::error::DeviceError;

    #[test]
    fn test_format_auth_error_message_includes_wallet_id() {
        let message = format_auth_error_message(&DeviceError::WalletNotFound, Some("device_123"), Some("wallet_456"));

        assert_eq!(message, "Wallet not found device_id=device_123 wallet_id=wallet_456");
    }
}
