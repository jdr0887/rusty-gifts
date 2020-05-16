CREATE TABLE gift_ideas (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  title VARCHAR(255) NOT NULL,
  description TEXT,
  price VARCHAR(255),
  url VARCHAR(255),
  date_added DATETIME NOT NULL,
  date_last_modified DATETIME NOT NULL,
  date_reserved DATETIME,
  owner_id INTEGER NOT NULL,
  recipient_user_id INTEGER NOT NULL,
  reserved_by_user_id INTEGER,
  FOREIGN KEY (owner_id) REFERENCES users (id),
  FOREIGN KEY (recipient_user_id) REFERENCES users (id),
  FOREIGN KEY (reserved_by_user_id) REFERENCES users (id)
)
