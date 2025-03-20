# Return String or &str

```rs
async fn handler() -> Html<String> {
    Html("<h1>Hello, Axum!</h1>".to_string()) // Convert &str to String
}


async fn handler() -> Html<&'static str> {
  Html("<h1>Hello, Axum!</h1>")
}
```

`Html<String>` for dynamic content, Useful for templating,

`Html<&'static str>` for static strings (better performance). No heap allocation (more efficient)
