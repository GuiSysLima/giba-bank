CREATE TYPE transaction_type AS ENUM ('DEPOSIT', 'TRANSFER');

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_from_id UUID REFERENCES accounts(id),
    account_to_id UUID NOT NULL REFERENCES accounts(id),
    amount DECIMAL(15,2) NOT NULL,
    transaction_type transaction_type NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);