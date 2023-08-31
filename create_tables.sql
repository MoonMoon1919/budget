PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS budgets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    total REAL
);

CREATE TABLE IF NOT EXISTS transactions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    value REAL,
    budget_id TEXT NOT NULL,
    FOREIGN KEY (budget_id)
       REFERENCES budgets (id)
       ON DELETE CASCADE
);

-- INSERT INTO budgets (id, name, total)
-- VALUES ("576bc364-7574-40ce-92ca-f488c613b7ea", "my-budget", 200.00);

-- INSERT INTO transactions (id, name, value, budget_id)
-- VALUES ("621dba84-399b-4846-8a9e-76a2d1692683", "cheeseborger", 3.99, "576bc364-7574-40ce-92ca-f488c613b7ea")

-- select * from budgets as b
-- join transactions as t ON b.id == t.budget_id
-- where b.id = "576bc364-7574-40ce-92ca-f488c613b7ea";
