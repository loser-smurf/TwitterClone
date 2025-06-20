// @generated automatically by Diesel CLI.

diesel::table! {
    follows (follower_id, followed_id) {
        follower_id -> Uuid,
        followed_id -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    likes (id) {
        id -> Uuid,
        user_id -> Uuid,
        tweet_id -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    tweets (id) {
        id -> Uuid,
        user_id -> Uuid,
        content -> Text,
        media_urls -> Nullable<Array<Nullable<Text>>>,
        reply_to_id -> Nullable<Uuid>,
        is_retweet -> Bool,
        original_tweet_id -> Nullable<Uuid>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Text,
        name -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(likes -> tweets (tweet_id));
diesel::joinable!(likes -> users (user_id));
diesel::joinable!(tweets -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(follows, likes, tweets, users,);
