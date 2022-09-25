DROP TABLE IF EXISTS posts CASCADE;
DROP TRIGGER IF EXISTS updated_at_trigger ON posts;
DROP INDEX IF EXISTS posts_num_idx;
DROP INDEX IF EXISTS posts_poster_id_idx;