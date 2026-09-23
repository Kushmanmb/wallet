use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::upsert::excluded;

use crate::DatabaseClient;
use crate::models::{NewScanDetectionRow, ScanDetectionRow};

pub(crate) trait ScanDetectionsStore {
    fn get_scan_detections(&mut self, targets: Vec<String>, since: NaiveDateTime) -> Result<Vec<ScanDetectionRow>, diesel::result::Error>;
    fn upsert_scan_detections(&mut self, values: Vec<NewScanDetectionRow>) -> Result<usize, diesel::result::Error>;
}

impl ScanDetectionsStore for DatabaseClient {
    fn get_scan_detections(&mut self, targets: Vec<String>, since: NaiveDateTime) -> Result<Vec<ScanDetectionRow>, diesel::result::Error> {
        use crate::schema::scan_detections::dsl::*;
        scan_detections.filter(target.eq_any(targets)).filter(updated_at.ge(since)).select(ScanDetectionRow::as_select()).load(&mut self.connection)
    }

    fn upsert_scan_detections(&mut self, values: Vec<NewScanDetectionRow>) -> Result<usize, diesel::result::Error> {
        use crate::schema::scan_detections::dsl::*;

        if values.is_empty() {
            return Ok(0);
        }

        diesel::insert_into(scan_detections)
            .values(values)
            .on_conflict((scan_type, chain, target))
            .do_update()
            .set((provider.eq(excluded(provider)), reason.eq(excluded(reason)), updated_at.eq(diesel::dsl::now)))
            .execute(&mut self.connection)
    }
}
