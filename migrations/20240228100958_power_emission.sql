CREATE TABLE power_emission (
    date_id TIMESTAMP WITHOUT TIME ZONE PRIMARY KEY,
    hard_coal DOUBLE PRECISION,
    lignite DOUBLE PRECISION,
    natural_gas DOUBLE PRECISION,
    other DOUBLE PRECISION,
    total_grid_emissions DOUBLE PRECISION
);
