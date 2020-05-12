CREATE TABLE gift_ideas (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  price VARCHAR(255),
  url VARCHAR(255),
  intended_for_user_id INTEGER,
  reserved_by_user_id INTEGER,
  FOREIGN KEY (intended_for_user_id) REFERENCES users (id),
  FOREIGN KEY (reserved_by_user_id) REFERENCES users (id)
)
