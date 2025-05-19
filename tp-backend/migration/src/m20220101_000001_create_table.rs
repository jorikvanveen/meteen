use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(pk_auto(Task::Id))
                    .col(boolean(Task::Done))
                    .col(string(Task::Description))
                    .col(ColumnDef::new(Task::Importance).float())
                    .col(ColumnDef::new(Task::Urgency).float())
                    .col(ColumnDef::new(Task::DueDate).timestamp())
                    .col(ColumnDef::new(Task::DoDate).timestamp())
                    .col(ColumnDef::new(Task::Category).integer().default(Value::Int(None)))
                    .foreign_key(ForeignKey::create().
                        name("FK_task_category")
                        .from(Task::Table, Task::Category)
                        .to(Category::Table, Category::Id)
                    )
                    .to_owned(),
            ).await?;
        manager.create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(pk_auto(Category::Id))
                    .col(string(Category::Name))
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await?;

        manager.drop_table(Table::drop().table(Category::Table).to_owned()).await

    }
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
    Name
}
#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    Done,
    Description,
    Importance,
    Urgency,
    DueDate,
    DoDate,
    Category
}
