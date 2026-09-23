use primitives::rewards::{USERNAME_MAX_LENGTH, USERNAME_MIN_LENGTH, is_custom_username};

use crate::error::UsernameValidationError;

pub fn validate_username(username: &str) -> Result<(), UsernameValidationError> {
    let length = username.len();
    if length < USERNAME_MIN_LENGTH {
        return Err(UsernameValidationError::Invalid(format!("Username must be at least {USERNAME_MIN_LENGTH} characters")));
    }
    if length > USERNAME_MAX_LENGTH {
        return Err(UsernameValidationError::Invalid(format!("Username must be at most {USERNAME_MAX_LENGTH} characters")));
    }
    if !username.chars().all(|character| character.is_ascii_alphanumeric()) {
        return Err(UsernameValidationError::Invalid("Username must contain only letters and digits".into()));
    }
    Ok(())
}

pub fn validate_username_available(is_taken: bool) -> Result<(), UsernameValidationError> {
    if is_taken { Err(UsernameValidationError::AlreadyTaken) } else { Ok(()) }
}

pub fn validate_wallet_without_username(current_username: &str) -> Result<(), UsernameValidationError> {
    if is_custom_username(current_username) {
        return Err(UsernameValidationError::Invalid("Wallet already has a username".into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        assert_eq!(validate_username("abcd"), Ok(()));
        assert_eq!(validate_username("user123"), Ok(()));
        assert_eq!(validate_username("1234567890123456"), Ok(()));
        assert_eq!(validate_username("abc"), Err(UsernameValidationError::Invalid("Username must be at least 4 characters".into())));
        assert_eq!(validate_username("12345678901234567"), Err(UsernameValidationError::Invalid("Username must be at most 16 characters".into())));
        for username in ["user_name", "user-name", "user.name", "user name"] {
            assert_eq!(validate_username(username), Err(UsernameValidationError::Invalid("Username must contain only letters and digits".into())));
        }
    }

    #[test]
    fn test_validate_username_available() {
        assert_eq!(validate_username_available(false), Ok(()));
        assert_eq!(validate_username_available(true), Err(UsernameValidationError::AlreadyTaken));
    }

    #[test]
    fn test_validate_wallet_without_username() {
        assert_eq!(validate_wallet_without_username("0x1234567890abcdef1234567890abcdef12345678"), Ok(()));
        assert_eq!(validate_wallet_without_username("alice"), Err(UsernameValidationError::Invalid("Wallet already has a username".into())));
    }
}
