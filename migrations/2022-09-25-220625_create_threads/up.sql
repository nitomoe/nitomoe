CREATE TABLE IF NOT EXISTS threads (
    "id" BIGSERIAL NOT NULL PRIMARY KEY,
    "subject" TEXT,
    "board_id" INT NOT NULL REFERENCES boards,
    "status" SMALLINT NOT NULL DEFAULT 0,
    "sticky" BOOLEAN NOT NULL DEFAULT FALSE,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER updated_at_trigger
    BEFORE UPDATE ON threads
    FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

CREATE INDEX IF NOT EXISTS threads_board_id_idx ON threads ("board_id");