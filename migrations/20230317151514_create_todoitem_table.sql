CREATE TYPE PRIORITY AS ENUM ('None', 'Low', 'Medium', 'High');

CREATE TABLE todo_items (
    id UUID PRIMARY KEY,
    list_id UUID NOT NULL,
    title VARCHAR(255),
    note TEXT,
    priority PRIORITY NOT NULL,
    reminder TIMESTAMP,
    done BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
/*    FOREIGN KEY (list_id) REFERENCES todo_lists(id)*/
);