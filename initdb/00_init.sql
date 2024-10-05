CREATE TABLE simulation_data (
    id SERIAL PRIMARY KEY,
    value_score FLOAT NOT NULL,
    high_score FLOAT NOT NULL,
    close_score FLOAT NOT NULL,
    custom_score FLOAT NOT NULL,

    value_x FLOAT NOT NULL,
    value_y FLOAT NOT NULL,

    high_x FLOAT NOT NULL,
    high_y FLOAT NOT NULL,

    close_x FLOAT NOT NULL,
    close_y FLOAT NOT NULL,

    custom_x FLOAT NOT NULL,
    custom_y FLOAT NOT NULL,

    timestamp TIMESTAMPTZ DEFAULT NOW()
);