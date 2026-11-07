use cookie::Cookie;

pub fn set_cookie(name: String, value: String, max_age: i64) -> Cookie<'static> {
    Cookie::build((name, value))
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(cookie::time::Duration::seconds(max_age))
        .build()
}
