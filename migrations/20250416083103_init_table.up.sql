-- Add up migration script here
CREATE TABLE users (
       id UUID PRIMARY KEY,
       username TEXT UNIQUE NOT NULL,
       email TEXT UNIQUE NOT NULL,
       password_hash TEXT NOT NULL,
       is_active BOOLEAN NOT NULL DEFAULT true,
       updated_at TIMESTAMP DEFAULT now(),
       created_at TIMESTAMP DEFAULT now()
);

-- Email Verification Tokens
CREATE TABLE verification_tokens (
       id BIGINT PRIMARY KEY AUTO_INCREMENT,
       user_id UUID NOT NULL,
       token VARCHAR(255)  NOT NULL,
       expires_at TIMESTAMP NOT NULL,
       created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
       FOREIGN KEY (user_id) REFERENCES users(id),
       UNIQUE(token)
);