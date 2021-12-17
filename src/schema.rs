table! {
    Blacklists (id) {
        id -> Integer,
        user_id -> BigInt,
        seller_id -> Integer,
    }
}

table! {
    Vahdit (id) {
        id -> Integer,
        url -> Text,
        user_id -> BigInt,
        last_updated -> BigInt,
    }
}

allow_tables_to_appear_in_same_query!(Blacklists, Vahdit,);
