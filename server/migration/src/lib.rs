pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20241206_000001_create_account;
mod m20241206_000002_create_transaction;
mod m20241206_000003_create_holdings;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241206_000001_create_account::Migration),
            Box::new(m20241206_000002_create_transaction::Migration),
            Box::new(m20241206_000003_create_holdings::Migration),
        ]
    }
}
