CREATE TABLE IF NOT EXISTS comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    userId INTEGER,
    tweetId INTEGER
    commentText TEXT NOT NULL,
    FOREIGN KEY (userId) REFERENCES users (id)
    FOREIGN KEY (tweetId) REFERENCES tweets (id)
);
