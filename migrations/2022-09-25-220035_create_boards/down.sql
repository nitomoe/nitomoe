DROP TABLE IF EXISTS boards CASCADE;
DROP TRIGGER IF EXISTS make_board_seq ON boards;
DROP TRIGGER IF EXISTS updated_at_trigger ON boards;
DROP FUNCTION IF EXISTS make_board_seq;
DROP INDEX IF EXISTS boards_name_unique_idx;