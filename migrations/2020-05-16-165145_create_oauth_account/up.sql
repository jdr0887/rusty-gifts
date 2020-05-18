CREATE TABLE oauthaccount (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  type VARCHAR(255) NOT NULL,
  external_id VARCHAR(255) NOT NULL,
  external_username VARCHAR(255),
  oauth_secret VARCHAR(255),
  oauth_token VARCHAR(255),
  picture_url VARCHAR(255),
  user_id: INTEGER,
  FOREIGN KEY (user_id) REFERENCES users (id),
)
