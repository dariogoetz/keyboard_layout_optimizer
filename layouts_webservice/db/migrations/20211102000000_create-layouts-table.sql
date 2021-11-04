CREATE TABLE layouts (
    id SERIAL PRIMARY KEY,
    layout VARCHAR NOT NULL,
    total_cost DOUBLE PRECISION NOT NULL,
    details_json VARCHAR,
    published_by VARCHAR,
    created TIMESTAMP,
    highlight BOOL
);
