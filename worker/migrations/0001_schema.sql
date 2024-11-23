-- Migration number: 0001 	 2024-11-23T10:42:55.874Z
CREATE TABLE "forms" (
  "id" integer PRIMARY KEY,
  "form_id" text NOT NULL UNIQUE,
  "template" text NOT NULL,
  "created_at" text NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "submissions" (
  "id" integer PRIMARY KEY,
  "form" integer REFERENCES "forms" ("id"),
  "submission_id" text NOT NULL UNIQUE,
  "encrypted_body" text NOT NULL,
  "created_at" text NOT NULL DEFAULT CURRENT_TIMESTAMP
);
