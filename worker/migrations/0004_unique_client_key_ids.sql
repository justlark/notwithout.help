-- Migration number: 0004 	 2025-02-13T07:01:44.376Z
ALTER TABLE "forms"
ADD COLUMN "next_key_index" integer NOT NULL DEFAULT 0;

UPDATE "forms"
SET
  "next_key_index" = (
    SELECT
      max("key_index") + 1
    FROM
      "keys"
    WHERE
      "keys"."form" = "forms"."id"
  )
WHERE
  EXISTS (
    SELECT
      1
    FROM
      "keys"
    WHERE
      "keys"."form" = "forms"."id"
  );

CREATE TRIGGER "update_next_key_index" AFTER INSERT ON "keys" FOR EACH ROW BEGIN
UPDATE "forms"
SET
  "next_key_index" = "next_key_index" + 1
WHERE
  "id" = NEW."form";

END;
