CREATE TABLE user_gift_ideas (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  user_id INTEGER NOT NULL,
  gift_idea_id INTEGER NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users (id),
  FOREIGN KEY (gift_idea_id) REFERENCES gift_ideas (id)
)
