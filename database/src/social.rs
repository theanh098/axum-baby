//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "social")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub last_update: Option<DateTime>,
    #[sea_orm(column_type = "Text", nullable)]
    pub twitter_id: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub twitter: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub discord_id: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub discord: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub telegram_id: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub telegram: Option<String>,
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}