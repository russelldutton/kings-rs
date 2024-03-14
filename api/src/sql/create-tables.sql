CREATE TABLE IF NOT EXISTS games
(
    id            INTEGER PRIMARY KEY,
    session_code  TEXT    UNIQUE       NOT NULL,
    player_host   TEXT    NOT NULL,
    is_started    BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS rounds
(
    id            INTEGER PRIMARY KEY,
    rank          TEXT  NOT NULL,
    hand_size     INTEGER  NOT NULL,
    game_id       INTEGER NOT NULL,
    FOREIGN KEY(game_id) REFERENCES games(game_id)
);

CREATE TABLE IF NOT EXISTS players
(
    id          INTEGER PRIMARY KEY,
    nickname    TEXT  NOT NULL,
    role        TEXT  NULL,
    game_id     INTEGER NULL,
    FOREIGN KEY(game_id) REFERENCES games(game_id)
);

CREATE TABLE IF NOT EXISTS turns
(
    id            INTEGER PRIMARY KEY,
    player_id     INTEGER  NOT NULL,
    round_id      INTEGER  NOT NULL,
    FOREIGN KEY(player_id) REFERENCES players(player_id),
    FOREIGN KEY(round_id) REFERENCES rounds(round_id)
);

CREATE TABLE IF NOT EXISTS cards
(
    id          INTEGER PRIMARY KEY,
    rank        TEXT  NOT NULL,
    suit        TEXT  NOT NULL,
    game_id     INTEGER,
    player_id   INTEGER,
    turn_id     INTEGER,
    FOREIGN KEY(game_id) REFERENCES games(game_id),
    FOREIGN KEY(player_id) REFERENCES players(player_id),
    FOREIGN KEY(turn_id) REFERENCES turns(turn_id)
);

CREATE TABLE IF NOT EXISTS swaps
(
    id            INTEGER PRIMARY KEY,
    session_code  TEXT  NOT NULL,
    player_host   INTEGER  NOT NULL
);