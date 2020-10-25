table! {
    user (id) {
        id -> Unsigned<Integer>,
        username -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
