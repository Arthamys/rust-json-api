-- Your SQL goes here
CREATE TABLE documents (
  id SERIAL PRIMARY KEY,
  title VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE revisions (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL,
  creation_date TIMESTAMP NOT NULL,
  document_id INTEGER NOT NULL,
  FOREIGN KEY(document_id) REFERENCES documents(id)
);
