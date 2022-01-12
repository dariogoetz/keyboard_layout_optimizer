-- Add migration script here
ALTER TABLE layouts
ADD layout_config VARCHAR NOT NULL DEFAULT 'standard';
