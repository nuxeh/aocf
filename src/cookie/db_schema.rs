table! {
    moz_cookies (id) {
        id -> Integer,
        originAttributes -> Text,
        name -> Text,
        value -> Text,
        host -> Text,
        path -> Text,
        expiry -> Integer,
        lastAccessed -> Integer,
        creationTime -> Integer,
        isSecure -> Integer,
        isHttpOnly -> Integer,
        inBrowserElement -> Integer,
        sameSite -> Integer,
    }
}
