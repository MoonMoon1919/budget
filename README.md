# Budget

A simple budget manager written in Rust

## RDD

Budget can has the following functionality:

- Add, remove, edit a budget
- Add, remove, edit transactions associated with a budget
- Display how much money is remaining in a budget
- Handle more than one budget

## Rough sketching

- A *user* can have many budgets
- A *budget* is created with a total dollar value (e.g., how much money it is allocated)
- A budget can have many transactions
- Each *transaction* has an associated dollar value and name
- The value of a transaction is removed _from the allocated budget_ max on add
- The value of a transaction is re-added _to the allocated budget_ max on removal
- The original value of a transaction is re-added, then the new value removed, _from the allocated budget_ max on value change

## SQLite

Create the DB with some seeded data

```bash
sqlite3 budgets.db < create_tables.sql
```
