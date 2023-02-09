-- Add migration script here

CREATE TABLE fish_type(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL,
    anishinaabe_name TEXT,
    fish_image TEXT,
    s3_fish_image TEXT,
    s3_woodland_image TEXT
);

CREATE TABLE fish(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    fish_type_id uuid NOT NULL,
    FOREIGN KEY (fish_type_id)
        REFERENCES fish_type (id),
    lake TEXT NOT NULL,
    date_sampled TIMESTAMP,
    mercury NUMERIC(7, 3),
    omega_3 NUMERIC(7, 3),
    omega_3_ratio NUMERIC(7, 3),
    pcb NUMERIC(7, 3),
    protein NUMERIC(7,3)
);

