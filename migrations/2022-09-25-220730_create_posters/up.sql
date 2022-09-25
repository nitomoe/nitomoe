CREATE TABLE IF NOT EXISTS posters (
    "id" BIGSERIAL NOT NULL PRIMARY KEY,
    "ip" INET NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER updated_at_trigger
    BEFORE UPDATE ON posters
    FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

CREATE INDEX posters_ip_idx ON posters ("ip");