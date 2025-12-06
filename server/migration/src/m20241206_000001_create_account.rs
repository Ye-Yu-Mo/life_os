use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(uuid(Account::Id).primary_key())
                    .col(uuid(Account::UserId).not_null())
                    .col(string_len(Account::Name, 100).not_null())
                    .col(string_len(Account::Type, 32).not_null())
                    .col(string_len(Account::CurrencyCode, 3).not_null())
                    .col(timestamp_with_time_zone(Account::CreatedAt).default(Expr::current_timestamp()).not_null())
                    .col(timestamp_with_time_zone(Account::UpdatedAt).default(Expr::current_timestamp()).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_account_user")
                            .from(Account::Table, Account::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_account_user")
                    .table(Account::Table)
                    .col(Account::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_account_user_type")
                    .table(Account::Table)
                    .col(Account::UserId)
                    .col(Account::Type)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_account_user_currency")
                    .table(Account::Table)
                    .col(Account::UserId)
                    .col(Account::CurrencyCode)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    UserId,
    Name,
    Type,
    CurrencyCode,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
