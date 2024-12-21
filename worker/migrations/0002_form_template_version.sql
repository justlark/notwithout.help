-- Migration number: 0002 	 2024-12-21T08:17:54.809Z
UPDATE forms
SET
  template = json_insert(template, '$.version', 1)
WHERE
  template ->> '$.version' IS NULL;
