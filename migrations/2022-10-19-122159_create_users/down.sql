-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS name ON update_users_time CASCADE;
DROP TABLE users;
DROP FUNCTION update_time_column();