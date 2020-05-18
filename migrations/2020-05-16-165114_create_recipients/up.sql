CREATE TABLE recipients (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  to_user_id INTEGER NOT NULL,
  from_user_id INTEGER NOT NULL,
  FOREIGN KEY (to_user_id) REFERENCES users (id),
  FOREIGN KEY (from_user_id) REFERENCES users (id)
)
