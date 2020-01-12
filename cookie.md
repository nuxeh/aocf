# AoC session cookie

To be able to retrieve personal input, and submit solutions, you need the
contents of your personal session cookie.

## Firefox

The `aocf` CLI can obtain this directly from the Firefox cookie store using
the `get-cookie` command, after previously logging in to Advent of Code using
Firefox.

To get the value manually, use "Storage Inspector" from the "Web Developer"
menu, and copy the value of the cookie named "session".

## Chrome

* Hit Ctrl-Shift-I for developer tools.
* Copy the "session" cookie text from the Application tab, as shown below:

![](cookie.png)
