-- This file should undo anything in `up.sql`

alter Table users COLUMN created_at timestamp;

insert into users created_at = now();