-- Add up migration script here

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS games
(
    id            INTEGER PRIMARY KEY,
    session_code  TEXT    UNIQUE     NOT NULL,
    player_host   INTEGER NOT NULL,
    status        TEXT    NOT NULL,
    FOREIGN KEY(player_host) REFERENCES players(id)
);

CREATE TABLE IF NOT EXISTS users
(
    id          INTEGER PRIMARY KEY,
    nick_name   TEXT  NOT NULL
);

CREATE TABLE IF NOT EXISTS rounds
(
    id            INTEGER PRIMARY KEY,
    rank          TEXT  NOT NULL,
    hand_size     INTEGER NOT NULL,
    game_id       INTEGER NOT NULL,
    FOREIGN KEY(game_id) REFERENCES games(id)
);

CREATE TABLE IF NOT EXISTS players
(
    id          INTEGER PRIMARY KEY,
    role        TEXT NULL,
    user_id     INTEGER NOT NULL,
    game_id     INTEGER NOT NULL,
    FOREIGN KEY(game_id) REFERENCES games(id),
    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS turns
(
    id            INTEGER PRIMARY KEY,
    player_id     INTEGER  NOT NULL,
    round_id      INTEGER  NOT NULL,
    FOREIGN KEY(player_id) REFERENCES players(id),
    FOREIGN KEY(round_id) REFERENCES rounds(id)
);

CREATE TABLE IF NOT EXISTS cards
(
    id          INTEGER PRIMARY KEY,
    rank        TEXT  NOT NULL,
    suit        TEXT  NOT NULL,
    game_id     INTEGER,
    player_id   INTEGER,
    turn_id     INTEGER,
    FOREIGN KEY(game_id) REFERENCES games(id),
    FOREIGN KEY(player_id) REFERENCES players(id),
    FOREIGN KEY(turn_id) REFERENCES turns(id)
);

CREATE TABLE IF NOT EXISTS swaps
(
    id            INTEGER PRIMARY KEY,
    session_code  TEXT  NOT NULL,
    player_host   INTEGER  NOT NULL
);