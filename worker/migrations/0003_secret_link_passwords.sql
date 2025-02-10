-- Migration number: 0003 	 2025-02-10T00:09:33.052Z
CREATE TABLE "passwords" (
  "id" integer PRIMARY KEY,
  "key" integer UNIQUE REFERENCES "keys" ("id") ON DELETE CASCADE,
  "salt" text NOT NULL,
  "nonce" text NOT NULL,
  "created_at" text NOT NULL DEFAULT CURRENT_TIMESTAMP
);
