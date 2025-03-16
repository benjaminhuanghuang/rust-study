# Mock

## Examples of Non-Deterministic Resources

- API
- External APls
- Databases
- Resources using global state
- Time-dependent resources
- Environmental factors

## Reasons for Mocking

- Control Over Test Environment
- Isolation of Components for Unit Testing
- Improved Test Stability and Reliability
- Efficiency and Speed
- Enabling Testing of Unavailable or Incomplete Components

## mockall crate

```sh
cargo add mockall
```

```rust
  let mut mock_api = MockWeatherApi::new();
  mock_api
    .expect_get_weather()
    .once()
    .returning(|_| "is cloudy today!".to_string());
```
