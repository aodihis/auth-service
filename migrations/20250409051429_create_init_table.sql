-- up.sql
CREATE TABLE users (
                       id UUID PRIMARY KEY,
                       username TEXT UNIQUE NOT NULL,
                       email TEXT UNIQUE NOT NULL,
                       password_hash TEXT NOT NULL,
                       is_active BOOLEAN NOT NULL DEFAULT true,
                       updated_at TIMESTAMP DEFAULT now(),
                       created_at TIMESTAMP DEFAULT now()
);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);

-- down.sql
DROP TABLE users;
