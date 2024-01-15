CREATE TABLE power_generation_and_consumption (
    date_id TIMESTAMP WITHOUT TIME ZONE PRIMARY KEY,
    biomass DOUBLE PRECISION,
    grid_emission_factor DOUBLE PRECISION,
    hard_coal DOUBLE PRECISION,
    hydro DOUBLE PRECISION,
    lignite DOUBLE PRECISION,
    natural_gas DOUBLE PRECISION,
    nuclear DOUBLE PRECISION,
    other DOUBLE PRECISION,
    pumped_storage_generation DOUBLE PRECISION,
    solar DOUBLE PRECISION,
    total_conventional_power_plant DOUBLE PRECISION,
    total_electricity_demand DOUBLE PRECISION,
    total_grid_emissions DOUBLE PRECISION,
    wind_offshore DOUBLE PRECISION,
    wind_onshore DOUBLE PRECISION
);
