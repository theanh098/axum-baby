// @generated automatically by Diesel CLI.

pub mod sql_types {
  #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "ActivityKind"))]
  pub struct ActivityKind;

  #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "BusinessStatus"))]
  pub struct BusinessStatus;

  #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "MediaSoucres"))]
  pub struct MediaSoucres;

  #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "ReviewStatuses"))]
  pub struct ReviewStatuses;

  #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "SuperUserRoles"))]
  pub struct SuperUserRoles;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ActivityKind;

    activity (id) {
        id -> Int4,
        created_at -> Timestamp,
        kind -> ActivityKind,
        user_id -> Int4,
        review_id -> Nullable<Int4>,
        point -> Int4,
        campaign_id -> Nullable<Int4>,
        platform_id -> Nullable<Text>,
    }
}

diesel::table! {
    banner (id) {
        id -> Int4,
        created_at -> Timestamp,
        expried_time -> Timestamp,
        source_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BusinessStatus;

    business (id) {
        id -> Int4,
        created_at -> Timestamp,
        name -> Varchar,
        overview -> Varchar,
        token -> Nullable<Varchar>,
        logo -> Nullable<Varchar>,
        founder_name -> Nullable<Varchar>,
        start_date -> Nullable<Timestamp>,
        address -> Nullable<Varchar>,
        whitepaper_url -> Nullable<Varchar>,
        contract_address -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
        types -> Nullable<Array<Nullable<Text>>>,
        main_category -> Varchar,
        chains -> Nullable<Array<Nullable<Text>>>,
        cmc_id -> Nullable<Int4>,
        contract_chain -> Nullable<Varchar>,
        status -> BusinessStatus,
        tags -> Nullable<Array<Nullable<Text>>>,
        creator_id -> Int4,
    }
}

diesel::table! {
    campaign (id) {
        id -> Int4,
        created_at -> Timestamp,
        title -> Varchar,
        description -> Varchar,
        metadata -> Nullable<Varchar>,
    }
}

diesel::table! {
    criteria_review (id) {
        id -> Int4,
        name -> Text,
        value -> Int4,
        review_id -> Int4,
    }
}

diesel::table! {
    did (id) {
        id -> Int4,
        controller -> Text,
        email -> Nullable<Text>,
        username -> Nullable<Text>,
    }
}

diesel::table! {
    email (id) {
        id -> Int4,
        created_at -> Timestamp,
        #[sql_name = "email"]
        mail -> Varchar,
    }
}

diesel::table! {
    follower_business (follower_id, business_id) {
        follower_id -> Int4,
        business_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MediaSoucres;

    media (id) {
        id -> Int4,
        created_at -> Timestamp,
        url -> Varchar,
        business_id -> Int4,
        path -> Nullable<Text>,
        source -> MediaSoucres,
    }
}

diesel::table! {
    notification (id) {
        id -> Int4,
        created_at -> Timestamp,
        business_id -> Nullable<Int4>,
        review_id -> Nullable<Int4>,
        seen -> Bool,
        to -> Int4,
        from -> Nullable<Int4>,
        meta_data -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::table! {
    rate_business (valuer_id, business_id) {
        valuer_id -> Int4,
        business_id -> Int4,
        rating -> Int4,
    }
}

diesel::table! {
    reply (id) {
        id -> Int4,
        created_at -> Timestamp,
        desc -> Varchar,
        review_id -> Int4,
        likes -> Nullable<Array<Nullable<Int4>>>,
        dislikes -> Nullable<Array<Nullable<Int4>>>,
        user_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ReviewStatuses;

    review (id) {
        id -> Int4,
        created_at -> Timestamp,
        rate -> Int4,
        business_id -> Int4,
        user_id -> Int4,
        status -> ReviewStatuses,
        likes -> Nullable<Array<Nullable<Int4>>>,
        dislikes -> Nullable<Array<Nullable<Int4>>>,
        headline -> Nullable<Varchar>,
        comment -> Nullable<Varchar>,
        txn_hash -> Nullable<Text>,
        sharings -> Nullable<Array<Nullable<Int4>>>,
    }
}

diesel::table! {
    search_param (id) {
        id -> Int4,
        business_name -> Text,
        times -> Int4,
    }
}

diesel::table! {
    social (id) {
        id -> Int4,
        last_update -> Nullable<Timestamp>,
        twitter_id -> Nullable<Text>,
        twitter -> Nullable<Text>,
        discord_id -> Nullable<Text>,
        discord -> Nullable<Text>,
        telegram_id -> Nullable<Text>,
        telegram -> Nullable<Text>,
        user_id -> Int4,
    }
}

diesel::table! {
    storage (id) {
        id -> Int4,
        created_at -> Timestamp,
        url -> Varchar,
        tag -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SuperUserRoles;

    super_user (id) {
        id -> Int4,
        role -> SuperUserRoles,
        refresh_token -> Nullable<Varchar>,
        username -> Varchar,
        password -> Varchar,
        avatar -> Nullable<Varchar>,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        created_at -> Timestamp,
        wallet_address -> Varchar,
        noti_accepted -> Bool,
        spam_accepted -> Bool,
        email -> Nullable<Varchar>,
        nickname -> Nullable<Varchar>,
        avatar_url -> Nullable<Varchar>,
        is_admin -> Bool,
        password -> Nullable<Varchar>,
        background_url -> Nullable<Varchar>,
        did_id -> Nullable<Int4>,
        last_sync_ibt -> Nullable<Timestamp>,
        last_update -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_campaign (user_id, campaign_id) {
        user_id -> Int4,
        campaign_id -> Int4,
        claimed -> Bool,
        amount -> Int4,
        txn_hash -> Nullable<Varchar>,
    }
}

diesel::joinable!(activity -> campaign (campaign_id));
diesel::joinable!(activity -> review (review_id));
diesel::joinable!(activity -> user (user_id));
diesel::joinable!(banner -> storage (source_id));
diesel::joinable!(business -> super_user (creator_id));
diesel::joinable!(criteria_review -> review (review_id));
diesel::joinable!(follower_business -> business (business_id));
diesel::joinable!(follower_business -> user (follower_id));
diesel::joinable!(media -> business (business_id));
diesel::joinable!(notification -> business (business_id));
diesel::joinable!(notification -> review (review_id));
diesel::joinable!(rate_business -> business (business_id));
diesel::joinable!(rate_business -> user (valuer_id));
diesel::joinable!(reply -> review (review_id));
diesel::joinable!(reply -> user (user_id));
diesel::joinable!(review -> business (business_id));
diesel::joinable!(review -> user (user_id));
diesel::joinable!(social -> user (user_id));
diesel::joinable!(user -> did (did_id));
diesel::joinable!(user_campaign -> campaign (campaign_id));
diesel::joinable!(user_campaign -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
  activity,
  banner,
  business,
  campaign,
  criteria_review,
  did,
  email,
  follower_business,
  media,
  notification,
  rate_business,
  reply,
  review,
  search_param,
  social,
  storage,
  super_user,
  user,
  user_campaign,
);
