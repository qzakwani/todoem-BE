BEGIN;

INSERT INTO users (id, username, email, name, password) 
VALUES ('c8686820-72ce-4391-bdce-e4f260dea40f', 'toto', 'toto@bobo.co', 'toto sasa', 'toto');

INSERT INTO users (id, username, email, password) 
VALUES ('5d43fc3c-8acb-48f9-9b25-8f8bd6f3d834', 'luffy', 'luffy@op.co', 'luffy');

INSERT INTO users (id, username, email, name, password) 
VALUES ('4157ee44-1de0-4168-a1f3-7ad6a5fd09b6', 'zoro', 'zoro@op.co', 'zoro japan', 'zoro');

INSERT INTO users (id, username, email, password) 
VALUES ('f6d1dabe-7766-4a6c-b34e-75e444cc3cbd', 'nami', 'nami@op.co', 'nami');


COMMIT;