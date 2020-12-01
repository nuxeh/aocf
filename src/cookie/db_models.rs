/*
PRAGMA table_info(moz_cookies)
0|id|INTEGER|0||1
2|originAttributes|TEXT|1|''|0
3|name|TEXT|0||0
4|value|TEXT|0||0
5|host|TEXT|0||0
6|path|TEXT|0||0
7|expiry|INTEGER|0||0
8|lastAccessed|INTEGER|0||0
9|creationTime|INTEGER|0||0
10|isSecure|INTEGER|0||0
11|isHttpOnly|INTEGER|0||0
12|inBrowserElement|INTEGER|0|0|0
13|sameSite|INTEGER|0|0|0
*/

#[allow(non_snake_case)]
#[derive(Queryable, Debug)]
pub struct FirefoxCookie {
    pub id: i32,
    pub originAttributes: String,
    pub name: String,
    pub value: String,
    pub host: String,
    pub path: String,
    pub expiry: i32,
    pub lastAccessed: i32,
    pub creationTime: i32,
    pub isSecure: i32,
    pub isHttpOnly: i32,
    pub inBrowserElement: i32,
    pub sameSite: i32,
}
