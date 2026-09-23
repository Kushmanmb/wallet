use diesel::{prelude::*, upsert::excluded};
use primitives::PlatformStore;

use crate::models::ReleaseRow;
use crate::sql_types::PlatformStore as PlatformStoreRow;
use crate::{DatabaseClient, DatabaseError};

pub trait ReleasesRepository {
    fn get_releases(&mut self) -> Result<Vec<ReleaseRow>, DatabaseError>;
    fn add_releases(&mut self, values: Vec<ReleaseRow>) -> Result<usize, DatabaseError>;
    fn update_release(&mut self, release: ReleaseRow) -> Result<usize, DatabaseError>;
    fn is_update_enabled(&mut self, store: PlatformStore) -> Result<bool, DatabaseError>;
}

impl ReleasesRepository for DatabaseClient {
    fn get_releases(&mut self) -> Result<Vec<ReleaseRow>, DatabaseError> {
        use crate::schema::releases::dsl::*;
        Ok(releases.order(updated_at.desc()).select(ReleaseRow::as_select()).load(&mut self.connection)?)
    }

    fn add_releases(&mut self, values: Vec<ReleaseRow>) -> Result<usize, DatabaseError> {
        use crate::schema::releases::dsl::*;
        Ok(diesel::insert_into(releases).values(&values).on_conflict_do_nothing().execute(&mut self.connection)?)
    }

    fn update_release(&mut self, release: ReleaseRow) -> Result<usize, DatabaseError> {
        use crate::schema::releases::dsl::*;
        Ok(diesel::insert_into(releases)
            .values(&release)
            .on_conflict(platform_store)
            .do_update()
            .set(version.eq(excluded(version)))
            .execute(&mut self.connection)?)
    }

    fn is_update_enabled(&mut self, store: PlatformStore) -> Result<bool, DatabaseError> {
        let store: PlatformStoreRow = store.into();
        use crate::schema::releases::dsl::*;
        let release = releases.filter(platform_store.eq(&store)).select(ReleaseRow::as_select()).first(&mut self.connection).optional()?;
        Ok(release.map(|r| r.update_enabled).unwrap_or(true))
    }
}
