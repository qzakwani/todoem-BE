BEGIN;


CREATE TABLE users (
  id UUID PRIMARY KEY,
  username VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);


CREATE TABLE IF NOT EXISTS user_connections (
user_id UUID REFERENCES users(id) NOT NULL ON DELETE CASCADE,
connection_id UUID REFERENCES users(id) NOT NULL ON DELETE CASCADE,
connected_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
PRIMARY KEY (user_id, connection_id)
);


CREATE TABLE IF NOT EXISTS user_connection_requests (
sender_id UUID REFERENCES users(id) NOT NULL ON DELETE CASCADE,
receiver_id UUID REFERENCES users(id) NOT NULL ON DELETE CASCADE,
sent_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
PRIMARY KEY (sender_id, receiver_id)
);

CREATE TYPE frequency AS ENUM ('daily', 'weekly', 'monthly');

CREATE TABLE IF NOT EXISTS tasks (
  id BIGSERIAL PRIMARY KEY,
  user_id UUID REFERENCES users(id) NOT NULL ON DELETE CASCADE,


  task TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  done BOOLEAN DEFAULT FALSE,
  due_date TIMESTAMP,
  repeat_frequency frequency,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS groups (
  id UUID PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS group_users(
  group_id UUID REFERENCES groups(id) NOT NULL ON DELETE CASCADE,
  user_id UUID REFERENCES users(id) NOT NULL ON DELETE CASCADE,
  is_admin BOOLEAN NOT NULL DEFAULT FALSE,
  joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  PRIMARY KEY (group_id, user_id)
);

CREATE TABLE IF NOT EXISTS group_tasks (
  id BIGSERIAL PRIMARY KEY,
  group_id UUID REFERENCES groups(id) NOT NULL ON DELETE CASCADE,

  task TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',

  done BOOLEAN NOT NULL DEFAULT FALSE,
  comment TEXT NOT NULL DEFAULT '',
  done_by_id UUID REFERENCES users(id) DEFAULT NULL ON DELETE SET NULL,
  done_at TIMESTAMP,

  updated_by_id UUID REFERENCES users(id) DEFAULT NULL ON DELETE SET NULL,
  updated_at TIMESTAMP,
  
  created_by_id UUID REFERENCES users(id) NOT NULL ON DELETE CASCADE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);


CREATE TABLE IF NOT EXISTS lists (
  id UUID PRIMARY KEY,
  user_id UUID REFERENCES users(id) NOT NULL ON DELETE CASCADE,

  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  task_count SMALLINT NOT NULL,
  done BOOLEAN NOT NULL DEFAULT FALSE,  

  sent_by_id UUID REFERENCES users(id) ON DELETE SET NULL,
  sent_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
);

CREATE TABLE IF NOT EXISTS list_tasks (
  id BIGSERIAL PRIMARY KEY,
  list_id UUID REFERENCES lists(id) NOT NULL ON DELETE CASCADE,

  task TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  done BOOLEAN NOT NULL DEFAULT FALSE,
);


CREATE TABLE IF NOT EXISTS sent_lists (
  id UUID PRIMARY KEY,
  user_id UUID REFERENCES users(id) NOT NULL ON DELETE CASCADE,

  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  task_count SMALLINT NOT NULL,

  sent_to_id UUID REFERENCES users(id) ON DELETE SET NULL,
  sent_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
);

CREATE TABLE IF NOT EXISTS list_tasks (
  id BIGSERIAL PRIMARY KEY,
  sent_list_id UUID REFERENCES sent_lists(id) NOT NULL ON DELETE CASCADE,

  task TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
);

COMMIT;