use chrono::NaiveDateTime;
use rewards::{NewRiskSignal, ReferralError, RiskScoreConfig, RiskScoringInput, RiskSignal, evaluate_risk};
use storage::models::{NewRiskSignalRow, RiskSignalRow};
use storage::{DatabaseClient, DatabaseError, RiskSignalsRepository};

pub enum RiskAssessment {
    Allowed { risk_signal_id: i32 },
    Exceeded { risk_signal_id: i32, error: ReferralError },
}

pub fn assess_referral_risk(client: &mut DatabaseClient, input: &RiskScoringInput, config: &RiskScoreConfig, since: NaiveDateTime) -> Result<Result<RiskAssessment, ReferralError>, DatabaseError> {
    if client.count_disabled_users_by_device(input.device_id, since)? > 0 {
        return Ok(Err(ReferralError::LimitReached));
    }

    let signal_input = input.to_signal_input();
    let fingerprint = signal_input.generate_fingerprint();
    if client.has_fingerprint_for_referrer(&fingerprint, &input.username, since).unwrap_or(false) {
        return Ok(Err(ReferralError::DuplicateAttempt));
    }

    let existing_signals: Vec<RiskSignal> = client
        .get_matching_risk_signals(&fingerprint, &signal_input.ip_address, &signal_input.ip_isp, &signal_input.device_model, signal_input.device_id, since)?
        .into_iter()
        .map(risk_signal)
        .collect();
    let device_model_ring_count = client.count_unique_referrers_for_device_model_pattern(&signal_input.device_model, signal_input.device_platform, &signal_input.device_locale, since)?;
    let ip_abuser_count = client.count_disabled_users_by_ip(&signal_input.ip_address, since)?;
    let cross_referrer_fingerprint_count = client.count_unique_referrers_for_fingerprint(&fingerprint, since)?;
    let referrer_country_count = client.count_unique_countries_for_referrer(&input.username, since)?;
    let referrer_device_count = client.count_unique_devices_for_referrer(&input.username, since)?;

    let risk_result = evaluate_risk(
        input,
        &existing_signals,
        device_model_ring_count,
        ip_abuser_count,
        cross_referrer_fingerprint_count,
        referrer_country_count,
        referrer_device_count,
        config,
    );
    let risk_signal_id = client.add_risk_signal(risk_signal_row(risk_result.signal))?;

    if !risk_result.score.is_allowed {
        return Ok(Ok(RiskAssessment::Exceeded {
            risk_signal_id,
            error: ReferralError::RiskScoreExceeded {
                score: risk_result.score.score,
                max_allowed: config.max_allowed_score,
            },
        }));
    }
    Ok(Ok(RiskAssessment::Allowed { risk_signal_id }))
}

fn risk_signal(row: RiskSignalRow) -> RiskSignal {
    RiskSignal {
        fingerprint: row.fingerprint,
        referrer_username: row.referrer_username,
        device_id: row.device_id,
        device_platform: *row.device_platform,
        device_model: row.device_model,
        ip_address: row.ip_address,
        ip_isp: row.ip_isp,
        created_at: row.created_at,
    }
}

fn risk_signal_row(signal: NewRiskSignal) -> NewRiskSignalRow {
    NewRiskSignalRow {
        fingerprint: signal.fingerprint,
        referrer_username: signal.referrer_username,
        device_id: signal.device_id,
        device_platform: signal.device_platform.into(),
        device_platform_store: signal.device_platform_store.into(),
        device_os: signal.device_os,
        device_model: signal.device_model,
        device_locale: signal.device_locale,
        device_currency: signal.device_currency,
        ip_address: signal.ip_address,
        ip_country_code: signal.ip_country_code,
        ip_usage_type: signal.ip_usage_type.into(),
        ip_isp: signal.ip_isp,
        ip_abuse_score: signal.ip_abuse_score,
        risk_score: signal.risk_score,
        user_agent: signal.user_agent,
        metadata: Some(signal.metadata),
    }
}
