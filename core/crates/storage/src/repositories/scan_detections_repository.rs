use std::time::Duration;

use chrono::Utc;
use primitives::ScanVerdict;

use crate::database::scan_detections::ScanDetectionsStore;
use crate::models::NewScanDetectionRow;
use crate::{DatabaseClient, DatabaseError};

pub trait ScanDetectionsRepository {
    fn get_scan_detections(&mut self, targets: Vec<String>, max_age: Duration) -> Result<Vec<ScanVerdict>, DatabaseError>;
    fn add_scan_detections(&mut self, values: Vec<ScanVerdict>) -> Result<usize, DatabaseError>;
}

impl ScanDetectionsRepository for DatabaseClient {
    fn get_scan_detections(&mut self, targets: Vec<String>, max_age: Duration) -> Result<Vec<ScanVerdict>, DatabaseError> {
        let since = Utc::now().naive_utc() - max_age;
        Ok(ScanDetectionsStore::get_scan_detections(self, targets, since)?.into_iter().map(|row| row.as_primitive()).collect())
    }

    fn add_scan_detections(&mut self, values: Vec<ScanVerdict>) -> Result<usize, DatabaseError> {
        Ok(ScanDetectionsStore::upsert_scan_detections(self, values.into_iter().map(NewScanDetectionRow::from_primitive).collect())?)
    }
}
