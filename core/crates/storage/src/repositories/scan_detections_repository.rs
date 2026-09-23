use std::time::Duration;

use chrono::Utc;

use crate::database::scan_detections::ScanDetectionsStore;
use crate::models::{NewScanDetectionRow, ScanDetectionRow};
use crate::{DatabaseClient, DatabaseError};

pub trait ScanDetectionsRepository {
    fn get_scan_detections(&mut self, targets: Vec<String>, max_age: Duration) -> Result<Vec<ScanDetectionRow>, DatabaseError>;
    fn add_scan_detections(&mut self, values: Vec<NewScanDetectionRow>) -> Result<usize, DatabaseError>;
}

impl ScanDetectionsRepository for DatabaseClient {
    fn get_scan_detections(&mut self, targets: Vec<String>, max_age: Duration) -> Result<Vec<ScanDetectionRow>, DatabaseError> {
        let since = Utc::now().naive_utc() - max_age;
        Ok(ScanDetectionsStore::get_scan_detections(self, targets, since)?)
    }

    fn add_scan_detections(&mut self, values: Vec<NewScanDetectionRow>) -> Result<usize, DatabaseError> {
        Ok(ScanDetectionsStore::upsert_scan_detections(self, values)?)
    }
}
