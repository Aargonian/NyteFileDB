CREATE TABLE files (
    sha256_hash        TEXT    NOT NULL,
    file_path          TEXT    NOT NULL,
    basename           TEXT    NOT NULL,
    size               INTEGER NOT NULL,
    extension          TEXT,
    file_type          TEXT    NOT NULL CHECK (file_type IN ('File', 'Symlink', 'Directory', 'Other')),
    mime_type          TEXT,
    creation_time      INTEGER NOT NULL,
    last_modified_time INTEGER NOT NULL,
    access_time        INTEGER NOT NULL,
    owner              TEXT    NOT NULL,
    permissions        INTEGER NOT NULL,
    PRIMARY KEY (sha256_hash, file_path)
);
