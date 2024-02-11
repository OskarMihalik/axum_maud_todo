--! update_todo (name, id)
UPDATE todo SET name=(:name) WHERE id=(:id);

--! delete_todo (id)
DELETE FROM todo WHERE id=(:id);

--! insert_todo (name)
INSERT INTO todo (name) values (:name);

--! select_todos
SELECT id, name FROM todo;