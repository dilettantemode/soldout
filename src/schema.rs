table! {
    user (id) {
        id -> Unsigned<Bigint>,
        username -> Varchar,
        email -> Varchar,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}
