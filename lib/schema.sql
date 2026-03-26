-- Enable foreign key constraints
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS NfcCard (
    uid           BLOB NOT NULL UNIQUE, -- [u8; 10]
    username      TEXT NOT NULL,
    membership_id INTEGER NOT NULL PRIMARY KEY UNIQUE,
    subscription_status    INTEGER NOT NULL,          -- 0 = inactive, 1 = active
    subscription_tier      INTEGER,                   -- 0 = BASIC, 1 = VIP
    subscription_start     INTEGER,                   -- Unix timestamp
    subscription_end       INTEGER,                   -- Unix timestamp
    last_used     INTEGER                    -- Unix timestamp
);

CREATE TABLE IF NOT EXISTS AuthToken (
    token       BLOB PRIMARY KEY NOT NULL, -- [u8; 32]
    uid         BLOB NOT NULL,             -- [u8; 10]
    FOREIGN KEY (uid) REFERENCES NfcCard(uid)
        ON DELETE CASCADE                  -- Optional: deletes tokens if card is deleted
);
