table! {
    documents (id) {
        id -> Int4,
        title -> Varchar,
    }
}

table! {
    revisions (id) {
        id -> Int4,
        content -> Text,
        creation_date -> Timestamp,
        document_id -> Int4,
    }
}

joinable!(revisions -> documents (document_id));

allow_tables_to_appear_in_same_query!(
    documents,
    revisions,
);
