CREATE TABLE IF NOT EXISTS NfcCard (
    uid         BLOB PRIMARY KEY,  -- [u8; 10]
    username    TEXT NOT NULL,
    user_id     INTEGER NOT NULL,
    sub_status  INTEGER NOT NULL,  -- 0 = inactive, 1 = active
    sub_start   INTEGER,           -- Unix timestamp, NULL if inactive
    sub_end     INTEGER,           -- Unix timestamp, NULL if inactive
    sub_tier    INTEGER,           -- 0 = BASIC, 1 = VIP
    last_used   INTEGER DEFAULT NULL            -- Unix timestamp, NULL if never used
);

CREATE TABLE IF NOT EXISTS AuthToken (
    token       BLOB PRIMARY KEY NOT NULL,  -- [u8; 32]
    uid         BLOB NOT NULL,             -- [u8; 10]
    FOREIGN KEY (uid) REFERENCES NfcCard(uid)
);
