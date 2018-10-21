CREATE TYPE unit AS ENUM ('unit');

CREATE TABLE settings (
  id unit PRIMARY KEY,
  username TEXT NOT NULL DEFAULT '',
  password TEXT NOT NULL DEFAULT '',
  info TEXT NOT NULL DEFAULT '',
  picture BYTEA NOT NULL DEFAULT '',
  upload_slots INTEGER NOT NULL DEFAULT 0,
  upload_rate INTEGER NOT NULL DEFAULT 0,
  download_slots INTEGER NOT NULL DEFAULT 0,
  download_rate INTEGER NOT NULL DEFAULT 0
)
