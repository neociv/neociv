# ![NEOCIV](https://raw.githubusercontent.com/neociv/neociv/master/logo.svg)

## neociv-db

High-level wrapper around all database functionality.

### Important

The database is designed to *automate* as much of the gameplay functionality as possible such that the "game" can be "played" via the sqlite CLI with much the same result as the actual game itself. Formatting checks, integrity, and triggers should all be provided with the database and not reliant on the module's code wherever possible. The prepared statements that are compiled into the client are still just regular SQL scripts that can be used directly.
