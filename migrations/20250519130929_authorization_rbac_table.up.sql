-- Add up migration script here
CREATE TABLE IF NOT EXISTS roles
(
    id
    SERIAL
    PRIMARY
    KEY,
    name
    VARCHAR
(
    50
) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                             );

-- Permissions table to store available permissions
CREATE TABLE IF NOT EXISTS permissions
(
    id
    SERIAL
    PRIMARY
    KEY,
    name
    VARCHAR
(
    100
) NOT NULL UNIQUE,
    description TEXT,
    resource VARCHAR
(
    100
) NOT NULL,
    action VARCHAR
(
    50
) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                             UNIQUE (resource, action)
    );

-- Role permissions junction table
CREATE TABLE IF NOT EXISTS role_permissions
(
    role_id
    INTEGER
    NOT
    NULL
    REFERENCES
    roles
(
    id
) ON DELETE CASCADE,
    permission_id INTEGER NOT NULL REFERENCES permissions
(
    id
)
  ON DELETE CASCADE,
    created_at TIMESTAMP
  WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
      PRIMARY KEY (role_id, permission_id)
    );

-- User roles junction table (assumes users table exists)
CREATE TABLE IF NOT EXISTS user_roles
(
    user_id
    INTEGER
    NOT
    NULL, -- References users(id) from the user table
    role_id
    INTEGER
    NOT
    NULL
    REFERENCES
    roles
(
    id
) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP
  WITH TIME ZONE,
      PRIMARY KEY (user_id, role_id)
    );


