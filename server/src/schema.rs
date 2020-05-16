table! {
    destinations (id) {
        id -> Integer,
        name -> Text,
        street -> Text,
        city -> Text,
        state -> Text,
        postal_code -> Text,
    }
}

table! {
    gift_ideas (id) {
        id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        price -> Nullable<Text>,
        url -> Nullable<Text>,
        date_added -> Timestamp,
        date_last_modified -> Timestamp,
        date_reserved -> Nullable<Timestamp>,
        owner_id -> Integer,
        recipient_user_id -> Integer,
        reserved_by_user_id -> Nullable<Integer>,
    }
}

table! {
    user_destinations (id) {
        id -> Integer,
        user_id -> Integer,
        destination_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        phone -> Nullable<Text>,
    }
}

joinable!(user_destinations -> destinations (destination_id));
joinable!(user_destinations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    destinations,
    gift_ideas,
    user_destinations,
    users,
);
