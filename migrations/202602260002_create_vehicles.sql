CREATE TABLE IF NOT EXISTS vehicles (
    id TEXT PRIMARY KEY,
    transport_company_id TEXT NOT NULL,
    make TEXT NOT NULL,
    model TEXT NOT NULL,
    year INTEGER NOT NULL,
    license_plate TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (transport_company_id) REFERENCES transport_companies(id) ON DELETE CASCADE
);
