-- Add migration script here
ALTER TABLE fish
    ALTER COLUMN mercury TYPE REAL,
    ALTER COLUMN pcb TYPE REAL,
    ALTER COLUMN omega_3 TYPE REAL,
    ALTER COLUMN protein TYPE REAL;
