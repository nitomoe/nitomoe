DROP TABLE IF EXISTS threads CASCADE;
DROP TRIGGER IF EXISTS updataed_at_trigger ON threads;
DROP INDEX IF EXISTS threads_board_id_idx;