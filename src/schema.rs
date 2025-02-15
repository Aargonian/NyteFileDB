// @generated automatically by Diesel CLI.

diesel::table! {
    files (sha256_hash, file_path) {
        sha256_hash -> Text,
        file_path -> Text,
        basename -> Text,
        size -> Integer,
        extension -> Nullable<Text>,
        file_type -> Text,
        mime_type -> Nullable<Text>,
        creation_time -> Integer,
        last_modified_time -> Integer,
        access_time -> Integer,
        owner -> Text,
        permissions -> Integer,
    }
}
