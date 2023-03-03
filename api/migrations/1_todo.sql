create table "transaction"
(
    transaction_id uuid primary key default gen_random_uuid(),
    lastname       text unique  not null
);