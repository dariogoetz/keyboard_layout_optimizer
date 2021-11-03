CREATE TABLE layouts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    layout VARCHAR NOT NULL,
    total_cost REAL NOT NULL,
    details_json VARCHAR
);
