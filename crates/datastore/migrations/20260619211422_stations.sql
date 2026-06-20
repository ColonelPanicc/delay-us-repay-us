CREATE TABLE IF NOT EXISTS stations (
    crs VARCHAR(3) PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    lat REAL NOT NULL,
    lon REAL NOT NULL
);

CREATE INDEX idx_stations_crs ON stations (crs);
CREATE INDEX idx_stations_location ON stations (lat, lon);
