-- Add migration script here
ALTER TABLE fish
    ALTER COLUMN omega_3_ratio TYPE REAL;
