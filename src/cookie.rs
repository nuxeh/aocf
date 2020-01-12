use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use failure::Error;
use std::path::Path;

fn connect_sqlite(path: impl AsRef<Path>) -> Result<SqliteConnection, Error> {
    let connection = SqliteConnection::establish(path.as_ref())?;
    Ok(connection)
}

pub fn get_session_cookie(path: impl AsRef<path>) -> Result<String, Error> {
    let connection = connect_sqlite(path)?;

    let result = urls::table
        .filter(FirefoxCookie::baseDomain.eq("adventofcode.com"))
        .filter(FirefoxCookie::name.eq("session"))
        .limit(1)
        .load(&connection)?;
}
