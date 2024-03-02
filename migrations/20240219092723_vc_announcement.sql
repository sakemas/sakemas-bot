CREATE TABLE IF NOT EXISTS idols (
    idol_id SERIAL PRIMARY KEY NOT NULL,
    idol_name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS announcements (
    announcement_id SERIAL PRIMARY KEY NOT NULL,
    content TEXT NOT NULL CHECK (CHAR_LENGTH(content) <= 1600),
    specified_date DATE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS announcement_idols (
    announcement_id INT NOT NULL,
    idol_id INT NOT NULL,
    PRIMARY KEY (announcement_id, idol_id),
    FOREIGN KEY (announcement_id) REFERENCES announcements (announcement_id) ON DELETE CASCADE,
    FOREIGN KEY (idol_id) REFERENCES idols (idol_id) ON DELETE CASCADE
);
