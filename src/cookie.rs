mod db_models;
mod db_schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use failure::Error;
use std::path::Path;

use db_models::*;
use db_schema::*;

fn connect_sqlite(path: impl AsRef<Path>) -> Result<SqliteConnection, Error> {
    let path = match path.as_ref().to_str() {
        Some(p) => p,
        None => bail!("can't parse path to string"),
    };
    let connection = SqliteConnection::establish(path)?;
    Ok(connection)
}

pub fn get_session_cookie(path: impl AsRef<Path>) -> Result<String, Error> {
    let connection = connect_sqlite(path)?;

    let records = moz_cookies::table
        .filter(moz_cookies::baseDomain.eq("adventofcode.com"))
        .filter(moz_cookies::name.eq("session"))
        .limit(1)
        .load::<FirefoxCookie>(&connection)?;

    if records.is_empty() {
        bail!("no cookie found in the cookie store");
    } else {
        Ok(records[0].value.to_owned())
    }
}
