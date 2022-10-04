CREATE TABLE IF NOT EXISTS files (
    "id" BIGSERIAL NOT NULL PRIMARY KEY,
    "filename" TEXT NOT NULL,
    "size" INT NOT NULL,
    "width" INT NOT NULL,
    "height" INT NOT NULL,
    "extension" TEXT NOT NULL,
    "post_id" BIGINT NOT NULL REFERENCES posts,
    -- Tbh just doing the tranditional hosted imageboard filename format.
    -- Another type of unique identifier would be better, but who cares
    "uid" BIGINT NOT NULL DEFAULT (EXTRACT(epoch FROM NOW()) * 1000000),
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER updated_at_trigger
    BEFORE UPDATE ON files
    FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

CREATE INDEX IF NOT EXISTS files_post_id_idx ON files ("post_id");
CREATE UNIQUE INDEX IF NOT EXISTS files_uid_idx ON files ("uid");