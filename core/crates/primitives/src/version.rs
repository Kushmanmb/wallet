pub fn is_version_higher(new: String, current: String) -> bool {
    let new: Vec<u32> = new.split('.').filter_map(|part| part.parse().ok()).collect();
    let current: Vec<u32> = current.split('.').filter_map(|part| part.parse().ok()).collect();
    for (new_part, current_part) in new.iter().zip(current.iter()) {
        if new_part != current_part {
            return new_part > current_part;
        }
    }
    new.len() > current.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_version_higher() {
        assert!(is_version_higher("1.2.3".into(), "1.0.0".into()));
        assert!(is_version_higher("2.1.3.4".into(), "2.1.3".into()));
        assert!(is_version_higher("0.1".into(), "0.0.1".into()));
        assert!(is_version_higher("2.27".into(), "2.0.27".into()));
        assert!(!is_version_higher("1.0.0".into(), "1.2.3".into()));
        assert!(!is_version_higher("1.2.3".into(), "1.2.3".into()));
        assert!(!is_version_higher("2.1.3".into(), "2.1.3.4".into()));
        assert!(!is_version_higher("1".into(), "2".into()));
        assert!(!is_version_higher("1.3.100".into(), "2.0.12".into()));
    }
}
