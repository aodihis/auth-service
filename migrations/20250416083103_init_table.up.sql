-- Add up migration script here
CREATE TABLE users (
        id UUID PRIMARY KEY,
        username TEXT UNIQUE NOT NULL,
        email TEXT UNIQUE NOT NULL,
        password_hash TEXT NOT NULL,
        email_verified BOOLEAN NOT NULL DEFAULT false,
        is_admin BOOLEAN NOT NULL DEFAULT FALSE,
        last_login TIMESTAMPTZ,
        updated_at TIMESTAMPTZ DEFAULT now(),
        created_at TIMESTAMPTZ DEFAULT now()
);

-- Email Verification Tokens
CREATE TABLE verification_tokens (
       user_id UUID NOT NULL,
       token VARCHAR(255)  NOT NULL,
       expires_at TIMESTAMPTZ NOT NULL,
       created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
       FOREIGN KEY (user_id) REFERENCES users(id),
       UNIQUE(token)
);