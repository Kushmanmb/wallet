use diesel::prelude::*;
use primitives::Chain;

use crate::models::ParserStateRow;
use crate::sql_types::ChainRow;
use crate::{DatabaseClient, DatabaseError, DieselResultExt};

pub trait ParserStateRepository {
    fn get_parser_state(&mut self, chain: Chain) -> Result<ParserStateRow, DatabaseError>;
    fn add_parser_state(&mut self, chain: Chain, block_time_ms: i32) -> Result<usize, DatabaseError>;
    fn get_parser_states(&mut self) -> Result<Vec<ParserStateRow>, DatabaseError>;
    fn set_parser_state_latest_block(&mut self, chain: Chain, block: i64) -> Result<usize, DatabaseError>;
    fn set_parser_state_current_block(&mut self, chain: Chain, block: i64) -> Result<usize, DatabaseError>;
}

impl ParserStateRepository for DatabaseClient {
    fn get_parser_state(&mut self, chain_value: Chain) -> Result<ParserStateRow, DatabaseError> {
        use crate::schema::parser_state::dsl::*;
        parser_state
            .filter(chain.eq(ChainRow::from(chain_value)))
            .select(ParserStateRow::as_select())
            .first(&mut self.connection)
            .or_not_found(chain_value.as_ref().to_string())
    }

    fn add_parser_state(&mut self, chain_value: Chain, block_time_ms: i32) -> Result<usize, DatabaseError> {
        use crate::schema::parser_state::dsl::*;
        Ok(diesel::insert_into(parser_state)
            .values((chain.eq(ChainRow::from(chain_value)), block_time.eq(block_time_ms)))
            .on_conflict_do_nothing()
            .execute(&mut self.connection)?)
    }

    fn get_parser_states(&mut self) -> Result<Vec<ParserStateRow>, DatabaseError> {
        use crate::schema::parser_state::dsl::*;
        Ok(parser_state.select(ParserStateRow::as_select()).load(&mut self.connection)?)
    }

    fn set_parser_state_latest_block(&mut self, chain_value: Chain, block: i64) -> Result<usize, DatabaseError> {
        use crate::schema::parser_state::dsl::*;
        Ok(diesel::update(parser_state.find(ChainRow::from(chain_value))).set(latest_block.eq(block)).execute(&mut self.connection)?)
    }

    fn set_parser_state_current_block(&mut self, chain_value: Chain, block: i64) -> Result<usize, DatabaseError> {
        use crate::schema::parser_state::dsl::*;
        Ok(diesel::update(parser_state.find(ChainRow::from(chain_value))).set(current_block.eq(block)).execute(&mut self.connection)?)
    }
}
