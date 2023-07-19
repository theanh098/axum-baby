use super::schema::sql_types;
use diesel_derive_enum::DbEnum;
use serde::Serialize;

#[derive(Debug, DbEnum)]
#[ExistingTypePath = "sql_types::ActivityKind"]
#[DbValueStyle = "snake_case"]
pub enum ActivityKind {
  Reviewapproved,
  Reacthelpful,
  Reactdownful,
  Reply,
  Share,
  JoinDiscord,
  JoinTelegram,
  Reward,
}
#[derive(Debug, DbEnum)]
#[ExistingTypePath = "sql_types::BusinessStatus"]
#[DbValueStyle = "snake_case"]
pub enum BusinessStatus {
  Approved,
  Pending,
  Rejected,
}

#[derive(Debug, DbEnum, Serialize)]
#[ExistingTypePath = "sql_types::MediaSoucres"]
#[DbValueStyle = "PascalCase"]
pub enum MediaSoucre {
  Photo,
  Telegram,
  Discord,
  Twitter,
  Blog,
}

#[derive(Debug, DbEnum)]
#[ExistingTypePath = "sql_types::ReviewStatuses"]
#[DbValueStyle = "snake_case"]
pub enum ReviewStatus {
  Approved,
  Pending,
  Rejected,
}

#[derive(Debug, DbEnum)]
#[ExistingTypePath = "sql_types::SuperUserRoles"]
#[DbValueStyle = "snake_case"]
pub enum SuperUserRole {
  Admin,
  Editor,
}
