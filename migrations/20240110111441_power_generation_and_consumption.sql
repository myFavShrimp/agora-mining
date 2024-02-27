CREATE TABLE power_generation (
    date_id TIMESTAMP WITHOUT TIME ZONE PRIMARY KEY,
    biomass DOUBLE PRECISION,
    hard_coal DOUBLE PRECISION,
    hydro DOUBLE PRECISION,
    lignite DOUBLE PRECISION,
    natural_gas DOUBLE PRECISION,
    nuclear DOUBLE PRECISION,
    other DOUBLE PRECISION,
    pumped_storage_generation DOUBLE PRECISION,
    solar DOUBLE PRECISION,
    total_conventional_power_plant DOUBLE PRECISION,
    wind_offshore DOUBLE PRECISION,
    wind_onshore DOUBLE PRECISION
);
