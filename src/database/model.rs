use diesel::{Identifiable, Queryable, Selectable};

#[derive(Queryable, Debug, Identifiable, Selectable)]
#[diesel(table_name = super::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub id: i32,
  // pub created_at: NaiveDateTime,
  pub wallet_address: String,
  pub noti_accepted: bool,
  pub spam_accepted: bool,
  pub email: Option<String>,
  pub nickname: Option<String>,
  pub avatar_url: Option<String>,
  pub is_admin: bool,
  pub password: Option<String>,
  pub background_url: Option<String>,
  pub did_id: Option<i32>,
  // pub last_sync_ibt: Option<NaiveDateTime>,
  // pub last_update: Option<NaiveDateTime>,
}
