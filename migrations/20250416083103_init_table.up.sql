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