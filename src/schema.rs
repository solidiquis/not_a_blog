table! {
    users (id) {
        id -> Int4,
        password -> Nullable<Varchar>,
        email -> Varchar,
        username -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}
