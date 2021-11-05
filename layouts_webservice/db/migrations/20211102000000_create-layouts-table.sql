CREATE TABLE layouts (
    id SERIAL PRIMARY KEY,
    layout VARCHAR NOT NULL,
    total_cost DOUBLE PRECISION NOT NULL,
    details_json VARCHAR NOT NULL,
    printed VARCHAR NOT NULL,
    published_by VARCHAR,
    created TIMESTAMP,
    highlight BOOL
);
