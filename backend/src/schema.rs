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
    gifts (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        price -> Nullable<Text>,
        url -> Nullable<Text>,
        intended_for_user_id -> Nullable<Integer>,
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
    user_gifts (id) {
        id -> Integer,
        user_id -> Integer,
        gift_id -> Integer,
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
joinable!(user_gifts -> gifts (gift_id));
joinable!(user_gifts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    destinations,
    gifts,
    user_destinations,
    user_gifts,
    users,
);
