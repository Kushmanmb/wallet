use diesel::prelude::*;
use diesel::upsert::excluded;

use crate::models::PriceProviderConfigRow;
use crate::{DatabaseClient, DatabaseError};

pub trait PricesProvidersRepository {
    fn add_prices_providers(&mut self, values: Vec<PriceProviderConfigRow>) -> Result<usize, DatabaseError>;
    fn get_prices_providers(&mut self) -> Result<Vec<PriceProviderConfigRow>, DatabaseError>;
}

impl PricesProvidersRepository for DatabaseClient {
    fn add_prices_providers(&mut self, values: Vec<PriceProviderConfigRow>) -> Result<usize, DatabaseError> {
        use crate::schema::prices_providers::dsl::*;
        Ok(diesel::insert_into(prices_providers)
            .values(&values)
            .on_conflict(id)
            .do_update()
            .set((priority.eq(excluded(priority)),))
            .execute(&mut self.connection)?)
    }

    fn get_prices_providers(&mut self) -> Result<Vec<PriceProviderConfigRow>, DatabaseError> {
        use crate::schema::prices_providers::dsl::*;
        Ok(prices_providers.order(priority.asc()).select(PriceProviderConfigRow::as_select()).load(&mut self.connection)?)
    }
}
