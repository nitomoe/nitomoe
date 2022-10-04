CREATE TYPE ThreadStatus AS ENUM (
    'Open', 'Locked'
);

CREATE TABLE IF NOT EXISTS threads (
    "id" BIGSERIAL NOT NULL PRIMARY KEY,
    "subject" TEXT,
    "board_id" INT NOT NULL REFERENCES boards,
    "status" ThreadStatus NOT NULL DEFAULT 'Open',
    "sticky" BOOLEAN NOT NULL DEFAULT FALSE,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER updated_at_trigger
    BEFORE UPDATE ON threads
    FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

CREATE INDEX IF NOT EXISTS threads_board_id_idx ON threads ("board_id");