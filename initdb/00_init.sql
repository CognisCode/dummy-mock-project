CREATE TABLE simulation_data (
    id SERIAL PRIMARY KEY,
    smart_score FLOAT NOT NULL,
    high_score FLOAT NOT NULL,
    close_score FLOAT NOT NULL,

    smart_start_x FLOAT NOT NULL,
    smart_start_y FLOAT NOT NULL,

    high_start_x FLOAT NOT NULL,
    high_start_y FLOAT NOT NULL,

    close_start_x FLOAT NOT NULL,
    close_start_y FLOAT NOT NULL,

    timestamp TIMESTAMPTZ DEFAULT NOW()
);