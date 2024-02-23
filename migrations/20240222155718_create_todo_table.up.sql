-- Add up migration script here
CREATE TABLE IF NOT EXISTS todo (
  id uuid NOT NULL DEFAULT gen_random_uuid(),
  value varchar(450) NOT NULL,
  active BOOLEAN NOT NULL DEFAULT FALSE,
  PRIMARY KEY (id)
)
