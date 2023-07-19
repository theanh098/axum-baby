use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "ActivityKind")]
pub enum ActivityKind {
  #[sea_orm(string_value = "reviewapproved")]
  Reviewapproved,
  #[sea_orm(string_value = "reacthelpful")]
  Reacthelpful,
  #[sea_orm(string_value = "reactdownful")]
  Reactdownful,
  #[sea_orm(string_value = "reply")]
  Reply,
  #[sea_orm(string_value = "share")]
  Share,
  #[sea_orm(string_value = "join_discord")]
  JoinDiscord,
  #[sea_orm(string_value = "join_telegram")]
  JoinTelegram,
  #[sea_orm(string_value = "reward")]
  Reward,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "BusinessStatus")]
pub enum BusinessStatus {
  #[sea_orm(string_value = "approved")]
  Approved,
  #[sea_orm(string_value = "pending")]
  Pending,
  #[sea_orm(string_value = "rejected")]
  Rejected,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "MediaSoucre")]
pub enum MediaSoucre {
  #[sea_orm(string_value = "Photo")]
  Photo,
  #[sea_orm(string_value = "Telegram")]
  Telegram,
  #[sea_orm(string_value = "Discord")]
  Discord,
  #[sea_orm(string_value = "Twitter")]
  Twitter,
  #[sea_orm(string_value = "Blog")]
  Blog,
}

#[derive(EnumIter, DeriveActiveEnum, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "tea")]
pub enum ReviewStatus {
  #[sea_orm(string_value = "approved")]
  Approved,
  #[sea_orm(string_value = "pending")]
  Pending,
  #[sea_orm(string_value = "rejected")]
  Rejected,
}

#[derive(EnumIter, DeriveActiveEnum, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "tea")]
pub enum SuperUserRole {
  #[sea_orm(string_value = "admin")]
  Admin,
  #[sea_orm(string_value = "editor")]
  Editor,
}
