-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '6fddf53e-5c32-42ee-ab05-9a367277b8e2',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$tCkDoZHcg676mQVqhNmbBw$DPw4nSkhlfRjRqYgZ4f0kNGEATY0Lan0vKO2Zf8nNOE'
);