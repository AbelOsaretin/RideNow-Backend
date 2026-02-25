CREATE TABLE IF NOT EXISTS driver_payments (
    id TEXT PRIMARY KEY,
    driver_id TEXT NOT NULL REFERENCES drivers(id) ON DELETE CASCADE,
    email TEXT NOT NULL,
    amount TEXT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'NGN',
    status TEXT NOT NULL,
    reference TEXT NOT NULL UNIQUE,
    authorization_url TEXT,
    access_code TEXT,
    gateway_response TEXT,
    raw_payload JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_driver_payments_driver_id ON driver_payments(driver_id);
CREATE INDEX IF NOT EXISTS idx_driver_payments_reference ON driver_payments(reference);
