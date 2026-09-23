use primitives::rewards::{RedemptionResponse, RedemptionResult};
use storage::{DatabaseClient, DatabaseError, RewardsRedemptionsRepository};

pub fn redeem_points(client: &mut DatabaseClient, username: &str, option_id: &str, device_id: i32, wallet_id: i32) -> Result<RedemptionResponse, DatabaseError> {
    let redemption = client.add_redemption(username, option_id, device_id, wallet_id)?;
    let redemption_id = redemption.id;
    Ok(RedemptionResponse {
        result: RedemptionResult { redemption },
        redemption_id,
    })
}
