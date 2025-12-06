use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(uuid(Transaction::Id).primary_key())
                    .col(uuid(Transaction::UserId).not_null())
                    .col(uuid(Transaction::FromAccountId))
                    .col(uuid(Transaction::ToAccountId))
                    .col(string_len(Transaction::TxnType, 16).not_null())
                    .col(decimal_len(Transaction::Amount, 18, 4).not_null())
                    .col(string_len(Transaction::CurrencyCode, 3).not_null())
                    .col(decimal_len(Transaction::ToAmount, 18, 4))
                    .col(string_len(Transaction::ToCurrencyCode, 3))
                    .col(string_len(Transaction::Category, 64))
                    .col(text(Transaction::Note))
                    .col(timestamp_with_time_zone(Transaction::OccurredAt).not_null())
                    .col(uuid(Transaction::RefTransactionId))
                    .col(string_len(Transaction::Merchant, 128))
                    .col(timestamp_with_time_zone(Transaction::CreatedAt).default(Expr::current_timestamp()).not_null())
                    .col(timestamp_with_time_zone(Transaction::UpdatedAt).default(Expr::current_timestamp()).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transaction_user")
                            .from(Transaction::Table, Transaction::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transaction_from_account")
                            .from(Transaction::Table, Transaction::FromAccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transaction_to_account")
                            .from(Transaction::Table, Transaction::ToAccountId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transaction_ref_txn")
                            .from(Transaction::Table, Transaction::RefTransactionId)
                            .to(Transaction::Table, Transaction::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE \"transaction\" ADD CONSTRAINT chk_amount_positive CHECK (amount > 0)"
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_transaction_user_occurred")
                    .table(Transaction::Table)
                    .col(Transaction::UserId)
                    .col(Transaction::OccurredAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_transaction_from_account")
                    .table(Transaction::Table)
                    .col(Transaction::FromAccountId)
                    .col(Transaction::OccurredAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_transaction_to_account")
                    .table(Transaction::Table)
                    .col(Transaction::ToAccountId)
                    .col(Transaction::OccurredAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_transaction_category")
                    .table(Transaction::Table)
                    .col(Transaction::Category)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_transaction_merchant")
                    .table(Transaction::Table)
                    .col(Transaction::Merchant)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_transaction_ref_txn")
                    .table(Transaction::Table)
                    .col(Transaction::RefTransactionId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transaction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Transaction {
    Table,
    Id,
    UserId,
    FromAccountId,
    ToAccountId,
    TxnType,
    Amount,
    CurrencyCode,
    ToAmount,
    ToCurrencyCode,
    Category,
    Note,
    OccurredAt,
    RefTransactionId,
    Merchant,
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
