CREATE TABLE IF NOT EXISTS idol_birthdays (
    birthday_id SERIAL PRIMARY KEY NOT NULL,
    idol_id INT NOT NULL,
    date DATE NOT NULL,
    FOREIGN KEY (idol_id) REFERENCES idols (idol_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS anniversaries (
    anniversary_id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    date DATE NOT NULL
);

CREATE TABLE IF NOT EXISTS calendar_idol_birthdays (
    date DATE NOT NULL,
    birthday_id INT NOT NULL,
    PRIMARY KEY (date, birthday_id),
    FOREIGN KEY (birthday_id) REFERENCES idol_birthdays (birthday_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS calendar_anniversaries (
    date DATE NOT NULL,
    anniversary_id INT NOT NULL,
    PRIMARY KEY (date, anniversary_id),
    FOREIGN KEY (anniversary_id) REFERENCES anniversaries (anniversary_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS anniversary_idols (
    anniversary_id INT NOT NULL,
    idol_id INT NOT NULL,
    PRIMARY KEY (anniversary_id, idol_id),
    FOREIGN KEY (anniversary_id) REFERENCES anniversaries (anniversary_id) ON DELETE CASCADE,
    FOREIGN KEY (idol_id) REFERENCES idols (idol_id) ON DELETE CASCADE
);
