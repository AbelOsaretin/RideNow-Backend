CREATE TABLE IF NOT EXISTS transport_company_drivers (
    id TEXT PRIMARY KEY,
    transport_company_id TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL,
    phone TEXT NOT NULL,
    license_number TEXT NOT NULL UNIQUE,
    vehicle_type TEXT NOT NULL,
    rating REAL,
    is_available BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (transport_company_id) REFERENCES transport_companies(id) ON DELETE CASCADE
);
