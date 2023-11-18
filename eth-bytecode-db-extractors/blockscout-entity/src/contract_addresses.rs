//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "contract_addresses")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub source_id: i64,
    pub created_at: DateTime,
    pub modified_at: DateTime,
    #[sea_orm(column_name = "_job_id")]
    pub job_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::job_queue::Entity",
        from = "Column::JobId",
        to = "super::job_queue::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    JobQueue,
}

impl Related<super::job_queue::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JobQueue.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
