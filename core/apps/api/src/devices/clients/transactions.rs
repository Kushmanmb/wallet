use std::error::Error;

use crate::params::MAX_QUERY_LIMIT;
use primitives::{AssetId, Transaction, TransactionId, TransactionsResponse};
use storage::models::TransactionRow;
use storage::{Database, DatabaseError, DevicesRepository, ScanAddressesRepository, TransactionsRepository, WalletsRepository};

use chrono::{DateTime, Utc};

pub struct TransactionsClient {
    database: Database,
}

impl TransactionsClient {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn get_transactions_by_wallet_id(
        &self,
        device_id: &str,
        device_row_id: i32,
        wallet_id: i32,
        asset_id: Option<AssetId>,
        from_timestamp: Option<u64>,
        limit: usize,
        offset: usize,
    ) -> Result<TransactionsResponse, Box<dyn Error + Send + Sync>> {
        let device_id = device_id.to_string();
        let from_datetime = from_timestamp.and_then(|ts| DateTime::<Utc>::from_timestamp(ts as i64, 0).map(|dt| dt.naive_utc()));
        Ok(self
            .database
            .run(move |client| {
                let subscriptions = client.get_subscriptions_by_wallet_id(device_row_id, wallet_id)?;
                let addresses = subscriptions.iter().map(|(_, addr)| addr.address.clone()).collect::<Vec<_>>();
                let chains = subscriptions.iter().map(|(sub, _)| sub.chain.0.as_ref().to_string()).collect::<Vec<_>>();
                let rows = client.get_transactions_by_device_id(&device_id, addresses.clone(), chains, asset_id, from_datetime, limit, offset)?;
                transactions_response(client, rows, addresses)
            })
            .await?)
    }

    pub async fn get_transactions_by_device_id(&self, device_id: &str) -> Result<TransactionsResponse, Box<dyn Error + Send + Sync>> {
        let device_id = device_id.to_string();
        Ok(self
            .database
            .run(move |client| {
                let device_row_id = client.get_device_row_id(&device_id)?;
                let subscriptions = client.get_subscriptions(device_row_id)?;
                let addresses = subscriptions.iter().map(|(_, _, addr)| addr.address.clone()).collect::<Vec<_>>();
                let chains = subscriptions.iter().map(|(_, sub, _)| sub.chain.0.as_ref().to_string()).collect::<Vec<_>>();

                if addresses.is_empty() || chains.is_empty() {
                    return Ok(TransactionsResponse::new(Vec::new(), Vec::new()));
                }

                let rows = client.get_transactions_by_device_id(&device_id, addresses.clone(), chains, None, None, MAX_QUERY_LIMIT, 0)?;
                transactions_response(client, rows, addresses)
            })
            .await?)
    }

    pub async fn get_transaction_by_id(&self, id: &TransactionId) -> Result<Transaction, Box<dyn Error + Send + Sync>> {
        let id = id.clone();
        let row = self.database.run(move |client| client.get_transaction_by_id(&id)).await?;
        Ok(row.as_primitive(vec![])?)
    }

    pub async fn get_transaction_by_wallet_id(&self, device_row_id: i32, wallet_id: i32, id: &TransactionId) -> Result<Transaction, Box<dyn Error + Send + Sync>> {
        let id = id.clone();
        let (addresses, row) = self
            .database
            .run(move |client| -> Result<_, DatabaseError> {
                let addresses = client.get_subscriptions_by_wallet_id(device_row_id, wallet_id)?.into_iter().map(|(_, address)| address.address).collect::<Vec<_>>();
                Ok((addresses, client.get_transaction_by_id(&id)?))
            })
            .await?;
        Ok(row.as_primitive(addresses.clone())?.finalize(addresses))
    }

    pub async fn get_transactions_by_hash(&self, hash: &str) -> Result<Vec<Transaction>, Box<dyn Error + Send + Sync>> {
        let hash = hash.to_string();
        let rows = self.database.run(move |client| client.get_transactions_by_hash(&hash)).await?;
        Ok(rows.into_iter().map(|row| row.as_primitive(vec![])).collect::<Result<Vec<_>, _>>()?)
    }
}

fn transactions_response(client: &mut impl ScanAddressesRepository, rows: Vec<TransactionRow>, addresses: Vec<String>) -> Result<TransactionsResponse, DatabaseError> {
    let transactions = rows
        .into_iter()
        .map(|row| row.as_primitive(addresses.clone()).map(|transaction| transaction.finalize(addresses.clone())))
        .collect::<Result<Vec<_>, _>>()?;

    let address_names = client
        .get_scan_addresses_by_addresses(transactions.iter().flat_map(|x| x.addresses()).collect())?
        .into_iter()
        .filter_map(|x| x.as_primitive())
        .collect();

    Ok(TransactionsResponse::new(transactions, address_names))
}
