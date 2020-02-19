table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        is_enabled -> Nullable<Bool>,
    }
}
