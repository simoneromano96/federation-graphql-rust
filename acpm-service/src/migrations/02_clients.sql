CREATE TABLE IF NOT EXISTS "clients" (
  "id" SERIAL PRIMARY KEY,
  "client_id" VARCHAR NOT NULL,
  "client_secret" VARCHAR NOT NULL
);
