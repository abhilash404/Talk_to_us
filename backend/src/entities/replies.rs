//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.13

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "replies")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub ticket_id: i32,
    pub agent_id: Option<i32>,
    #[sea_orm(column_type = "Text")]
    pub reply_text: String,
    pub created_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
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
        from = "Column::AgentId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
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
