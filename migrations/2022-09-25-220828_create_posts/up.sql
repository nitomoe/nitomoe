CREATE TABLE IF NOT EXISTS posts (
    "id" BIGSERIAL NOT NULL PRIMARY KEY,
    "num" BIGINT NOT NULL,
    "thread_id" BIGINT NOT NULL REFERENCES threads,
    "poster_id" BIGINT NOT NULL REFERENCES posters,
    "body" TEXT,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX posts_num_idx ON posts ("num");
CREATE INDEX posts_poster_id_idx ON posts("poster_id");

CREATE OR REPLACE FUNCTION fill_in_post_num_seq() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
    DECLARE
        loc_board_id INT;
BEGIN
    -- Avoid potentially unnecessary SELECT? Would be nice!
    SELECT board_id INTO loc_board_id FROM "threads" WHERE "id" = NEW.thread_id;
    NEW.num := nextval('board_seq_' || loc_board_id);
    RETURN NEW;
END
$$;

CREATE TRIGGER fill_in_post_num_seq BEFORE INSERT ON posts FOR EACH ROW EXECUTE PROCEDURE fill_in_post_num_seq();

CREATE TRIGGER updated_at_trigger
    BEFORE UPDATE ON posts
    FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();