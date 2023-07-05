-- This table contains some general information about a user.
CREATE TABLE "users" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(50) NOT NULL,
    "email" VARCHAR(50) DEFAULT NULL,
    "phone_number_code" INT NOT NULL,
    "phone_number" VARCHAR(15) NOT NULL,
    "password" VARCHAR(50) DEFAULT NULL,
    "token" VARCHAR(200) DEFAULT NULL,
    "verified" BOOLEAN DEFAULT FALSE NOT NULL
);

/* 
    This table contains some roles that could be
    assigned to the users.
*/
CREATE TABLE "roles" (
    "id" SERIAL PRIMARY KEY,
    "title" VARCHAR(50) NOT NULL,
    "description" TEXT 
);

-- This table joins "users" table with "roles" table.
CREATE TABLE "users_roles" (
    "id" SERIAL PRIMARY KEY,
    "user_id" INT NOT NULL,
    "role_id" INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES "users" (id),
    FOREIGN KEY (role_id) REFERENCES "roles" (id)
);


/*
    Insert several default roles in the database.
*/
INSERT INTO "roles" ("title", "description")
VALUES
    ('User', 'A general application user'),
    ('Admin', 'A user with a rather high access level'),
    ('Manager', 'A user with super high access level');