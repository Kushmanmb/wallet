use diesel::prelude::*;
use diesel::upsert::excluded;

use crate::models::{NewSupportSessionRow, SupportSessionRow};
use crate::{DatabaseClient, DatabaseError};

pub trait SupportSessionsRepository {
    fn get_support_session(&mut self, device_id: i32) -> Result<Option<SupportSessionRow>, DatabaseError>;
    fn set_support_session(&mut self, value: NewSupportSessionRow) -> Result<SupportSessionRow, DatabaseError>;
}

impl SupportSessionsRepository for DatabaseClient {
    fn get_support_session(&mut self, device_id_value: i32) -> Result<Option<SupportSessionRow>, DatabaseError> {
        use crate::schema::support_sessions::dsl::*;
        Ok(support_sessions.filter(device_id.eq(device_id_value)).select(SupportSessionRow::as_select()).first(&mut self.connection).optional()?)
    }

    fn set_support_session(&mut self, value: NewSupportSessionRow) -> Result<SupportSessionRow, DatabaseError> {
        use crate::schema::support_sessions::dsl::*;
        Ok(diesel::insert_into(support_sessions)
            .values(&value)
            .on_conflict(device_id)
            .do_update()
            .set(auth_token.eq(excluded(auth_token)))
            .returning(SupportSessionRow::as_returning())
            .get_result(&mut self.connection)?)
    }
}
