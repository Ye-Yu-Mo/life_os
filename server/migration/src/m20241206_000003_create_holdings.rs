use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Holdings::Table)
                    .if_not_exists()
                    .col(uuid(Holdings::Id).primary_key())
                    .col(uuid(Holdings::UserId).not_null())
                    .col(uuid(Holdings::AccountId).not_null())
                    .col(string_len(Holdings::AssetType, 16).not_null())
                    .col(string_len(Holdings::Symbol, 32).not_null())
                    .col(string_len(Holdings::Name, 128))
                    .col(decimal_len(Holdings::Quantity, 24, 8).not_null().default(0))
                    .col(decimal_len(Holdings::CostBasisTotal, 18, 4).not_null().default(0))
                    .col(string_len(Holdings::CurrencyCode, 3).not_null())
                    .col(decimal_len(Holdings::LastPrice, 18, 6))
                    .col(timestamp_with_time_zone(Holdings::LastPriceAt))
                    .col(decimal_len(Holdings::MarketValue, 18, 4))
                    .col(timestamp_with_time_zone(Holdings::CreatedAt).default(Expr::current_timestamp()).not_null())
                    .col(timestamp_with_time_zone(Holdings::UpdatedAt).default(Expr::current_timestamp()).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_holdings_user")
                            .from(Holdings::Table, Holdings::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_holdings_account")
                            .from(Holdings::Table, Holdings::AccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE holdings ADD CONSTRAINT chk_quantity_positive CHECK (quantity >= 0)"
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE holdings ADD CONSTRAINT chk_cost_basis_positive CHECK (cost_basis_total >= 0)"
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_holdings_user")
                    .table(Holdings::Table)
                    .col(Holdings::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_holdings_account")
                    .table(Holdings::Table)
                    .col(Holdings::AccountId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_holdings_asset_type")
                    .table(Holdings::Table)
                    .col(Holdings::AssetType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_holdings_user_symbol")
                    .table(Holdings::Table)
                    .col(Holdings::UserId)
                    .col(Holdings::Symbol)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uk_holdings_user_account_asset_symbol")
                    .table(Holdings::Table)
                    .col(Holdings::UserId)
                    .col(Holdings::AccountId)
                    .col(Holdings::AssetType)
                    .col(Holdings::Symbol)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Holdings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Holdings {
    Table,
    Id,
    UserId,
    AccountId,
    AssetType,
    Symbol,
    Name,
    Quantity,
    CostBasisTotal,
    CurrencyCode,
    LastPrice,
    LastPriceAt,
    MarketValue,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
