CREATE TABLE IF NOT EXISTS boards (
    "id" SERIAL NOT NULL PRIMARY KEY,
    "title" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE FUNCTION make_board_seq() RETURNS TRIGGER
    LANGUAGE plpgsql
    AS $$
BEGIN
    execute format('CREATE SEQUENCE board_seq_%s', NEW.id);
    return NEW;
END
$$;

CREATE TRIGGER make_board_seq AFTER INSERT ON boards FOR EACH ROW EXECUTE PROCEDURE make_board_seq();
CREATE TRIGGER updated_at_trigger
    BEFORE UPDATE ON boards
    FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

CREATE UNIQUE INDEX IF NOT EXISTS boards_name_unique_idx ON boards ("name");