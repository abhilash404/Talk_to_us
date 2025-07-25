//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.13

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "attachments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub ticket_id: Option<i32>,
    pub message_id: Option<i32>,
    pub file_path: String,
    pub file_name: String,
    pub uploaded_by: i32,
    pub uploaded_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::messages::Entity",
        from = "Column::MessageId",
        to = "super::messages::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Messages,
    #[sea_orm(
        belongs_to = "super::tickets::Entity",
        from = "Column::TicketId",
        to = "super::tickets::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Tickets,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UploadedBy",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<super::messages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messages.def()
    }
}

impl Related<super::tickets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tickets.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
