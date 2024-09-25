CREATE TABLE simulation_data (
    id SERIAL PRIMARY KEY,
    smartscore FLOAT NOT NULL,
    high_score FLOAT NOT NULL,
    close_score FLOAT NOT NULL,
    timestamp TIMESTAMPTZ DEFAULT NOW()
);