//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use crate::native_enum::ActivityKind;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "activity")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  pub created_at: DateTime,
  pub user_id: i32,
  pub review_id: Option<i32>,
  pub point: i32,
  pub campaign_id: Option<i32>,
  #[sea_orm(column_type = "Text", nullable)]
  pub platform_id: Option<String>,
  pub kind: ActivityKind,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::campaign::Entity",
    from = "Column::CampaignId",
    to = "super::campaign::Column::Id",
    on_update = "Cascade",
    on_delete = "SetNull"
  )]
  Campaign,
  #[sea_orm(
    belongs_to = "super::review::Entity",
    from = "Column::ReviewId",
    to = "super::review::Column::Id",
    on_update = "Cascade",
    on_delete = "SetNull"
  )]
  Review,
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id",
    on_update = "Cascade",
    on_delete = "Restrict"
  )]
  User,
}

impl Related<super::campaign::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Campaign.def()
  }
}

impl Related<super::review::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Review.def()
  }
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}