# Unit Testing

Whether you prefer test-first (TDD) or test-later, unit tests are a good idea! Rust has a build-in unit testing framework, and it's easy to use.

## Simple Unit Tests

When you make a library with `cargo new --lib`, Rust presents you with a simple unit test. You can run tests at any time by typing `cargo test`. You can also use `cargo test --all` to test the entire workspace.

Let's put together a very simple set of tests just to illustrate the basics. You've probably already run into this, so we'll keep it brief.

```rust
pub fn double_overflow(n: i32) -> i32 {
    n.overflowing_mul(2).0
}

#[cfg(test)]
mod test {
    use super::*;

    // Test for a positive number that doesn't cause an overflow
    #[test]
    fn test_double_overflow_positive() {
        assert!(double_overflow(5) == 10);
    }

    #[test]
    fn test_double_overflow_positive_overflow() {
        let result = double_overflow(i32::MAX);
        assert_eq!(result, -2);
    }

    #[test]
    fn test_double_overflow_inequality_positive() {
        let result = double_overflow(7);
        assert_ne!(result, 8); // 7 * 2 is 14, not 8
    }
}
```

This is pretty straightforward, and illustrates the test syntax - and the `assert` macro variants. (`assert`, `assert_eq`, `assert_ne`).

### Testing for Panics

You can test that a function panics by using `should_panic`:

```rust
#[test]
#[should_panic]
fn test_panic() {
    panic!("Oops");
}
```

### The ? Operator in Tests

You can write tests that return a `Result`, allowing you to use the `?` operator in your tests. With the following function:

```rust
pub fn double_safe(n: i32) -> Result<i32, String> {
    let (result, overflow) = n.overflowing_mul(2);
    if overflow {
        Err("overflow".to_string())
    } else {
        Ok(result)
    }
}
```

You can write the following test:

```rust
#[test]
fn test_overflow_detection() -> Result<(), String> {
    double_safe(i32::MAX)?;
    Ok(())
}
```

## Async Unit Tests with Tokio

The default Rust setup doesn't support unit-testing async tests. Just like normal async execution, you need an executor to manage the async tasks. Tokio provides a test framework that does this for you. Add Tokio to the unit test suite with `cargo add tokio -F full`

Now you can create your async functions that require testing:

```rust
pub async fn async_double(n: i32) -> i32 {
    n * 2
}
```

And you can use the `#[tokio::test]` macro to include a runtime in your tests:

```rust
#[tokio::test]
async fn test_async_double() {
    assert_eq!(4, async_double(2).await);
}
```

By default, tests run in a single-threaded context. If you do want to test in a threaded runtime, you can decorate the macro:

```rust
#[tokio::test(flavor = "multi_thread")]
```

You can constrain the number of threads, also:

```rust
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
```

See [the Tokio documentation](https://docs.rs/tokio-macros/latest/tokio_macros/attr.test.html) for more information.

## SQLx Testing

If you're using SQLX, it includes a suite of test macros also. The `[sqlx::test]` macro is the same as `[tokio::test]`---it creates a Tokio runtime for you (assuming you configured SQLx to use Tokio).

Let's add Tokio and Sqlite to our project:

```toml
[dependencies]
sqlx = { version = "0.7.1", features = ["sqlite", "runtime-tokio"] }
```

We'll *not* configure a database URL, and just use in-memory SQLite for now.

Let's create a migration that just creates a table:

```bash
sqlx migrate add create_messages_table
```

And we'll edit the resulting migration to create the same table we used before:

```sql
CREATE TABLE IF NOT EXISTS messages
(
    id          INTEGER PRIMARY KEY NOT NULL,
    message     TEXT                NOT NULL
);
```

Since we're working in-memory, we don't have to worry about data lingering from other tests yet. Here's a working test:

```rust
#[sqlx::test]
async fn test_insert(pool: sqlx::SqlitePool) -> sqlx::Result<()> {
    use sqlx::{Executor, Row};
    let mut conn = pool.acquire().await?;

    conn.execute("INSERT INTO messages (id, message) VALUES (1, 'Hello')")
        .await?;

    let rows = sqlx::query("SELECT * FROM messages")
        .fetch_all(&pool)
        .await?;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<i64, _>("id"), 1);
    assert_eq!(rows[0].get::<String, _>("message"), "Hello");
    Ok(())
}
```

Notice how the `test` macro can insert a connection pool. Your `.env` file (or environment) variable can be configured to connect to a real database. Any migrations will be run before the tests are executed.

> Just an aside, be careful to separate test from production databases. I once accidentally replaced every news article on a live website with the word "quilting". That was not a fun day!

### Test Fixtures

Sometimes you need some existing data to test. SQLX supports this as "fixtures". Create a new directory in your `src` directory named `fixtures`. In the new directory, create a file named `some_messages.sql`.

Populate it with:

```sql
INSERT INTO messages (id, message) VALUES (1, 'Hello World!');
INSERT INTO messages (id, message) VALUES (2, 'Hello Galaxy!');
INSERT INTO messages (id, message) VALUES (3, 'Hello Universe!');
```

Now you can decorate your macro to automatically run the `fixtures` script when you start:

```rust
#[sqlx::test(fixtures("some_messages"))]
async fn test_fixture(pool: sqlx::SqlitePool) -> sqlx::Result<()> {
    use sqlx::Row;
    let rows = sqlx::query("SELECT * FROM messages WHERE id=1")
        .fetch_all(&pool)
        .await?;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<i64, _>("id"), 1);
    assert_eq!(rows[0].get::<String, _>("message"), "Hello World!");
    Ok(())
}
```

## Testing Axum Services

Axum also provides you with some test infrastructure. Let's add `axum` to our project (`cargo add axum`) and `serde_json` (`cargo add serde_json`). You'll also need to add `tower`, `mime` and `hyper` as a dev-dependency (test-only):

```toml
[dev-dependencies]
tower = { version = "0.4.13", features = ["util"] }
hyper = { version = "0.14", features = ["full"] }
mime = "0.3"
```

Next, we build a service:

```rust
pub fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/json",
            post(|payload: Json<serde_json::Value>| async move {
                Json(serde_json::json!({ "data": payload.0 }))
            }),
        )
}
```

This returns "Hello, World!" to requests to `/`, and some JSON to a post to `/json`.

Let's test "Hello World":

```rust
#[tokio::test]
async fn test_hello_world() {
    let app = app();

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"Hello, World!");
}
```

For simple tests, you can bypass the network by calling into "oneshot" to build a request directly, and then checking the response. You're building the entire Axum service stack and testing it. For huge stacks, you may want to construct parts of them for your test---but be careful to make sure they remain in-sync.

Let's test the POST request:

```rust
#[tokio::test]
async fn test_json() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/json")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!([1, 2, 3, 4])).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, serde_json::json!({ "data": [1, 2, 3, 4] }));
}
```

> I do recommend using some automation for web testing, in addition to unit tests. Selenium and its ilk are great for this.

## Documentation and Testing

Rust includes a full documentation system, that includes testing your documentation. I strongly recommend using Rust's documentation system---especially for any code you have to maintain or share with others. Here's an example of a fully documented function:

```rust
/// Triple a number
/// 
/// # Arguments
/// * `n` - The number to triple
/// 
/// # Returns
/// The tripled number
/// 
/// # Examples
/// ```
/// assert_eq!(simple_unit_tests::triple(2), 6);
/// ```
pub fn triple(n: i32) -> i32 {
    n * 3
}
```

If you run `cargo test` with the documentation in place, the example code is also tested. By default, Rust will test any code in code blocks (triple backticks).

If you **don't** want to test an example (because its destructive, requires external resources, etc) you can write:

```rust
/// ```no_run
/// (your code)
/// ```
```

If you don't want to even see if the example compiles, you can use:

```rust
/// ```ignore
/// (your code)
/// ```
```

## Stubbing, Faking and Mocking

When testing, you sometimes want to test in isolation without a real implementation of other modules. Note that you absolutely should integration test as well!

* "Stubbing" is replacing a complicated type with a simplified local type for testing. A "stub" is a test double, pretending to be the real thing - but with simplified assumptions.
* "Faking" replaces a type with a fake result. You might fake the results of another system altogether.
* "Mocking" tries to retain the interface of a type or trait, while still "faking it".

### Faking (See also property testing, below)

The easiest way to fake is just to use an assumed value with a comment where it would have come from. That's not very clear, but it gets the job done! The [fake_rs](https://github.com/cksac/fake-rs) crate is really handy for generating fake test data.

### Stubbing

The simplest possible form of stubbing uses compiler directives to replace types at build-time with a `cfg(test)` version. For example:

```rust
#[cfg(not(test))]
pub fn complex_math() -> i32 {
    4 * 3 // Let's pretend that's complex
}

#[cfg(test)]
pub fn complex_math() -> i32 {
    12
}
```

Wrap your function that tests external functionality with a stub-version, and you can test the stub.

You can do this with types, even those imported from elsewhere. Create a new file, `stubbing.rs` and put a type into it:

```rust
pub struct StubMe;

impl StubMe {
    pub fn new() {
        Self
    }

    pub fn do_something() {
        // Do something
    }
}
```

Now in `lib.rs`, you can "stub in" a test version. In the test section:

```rust
struct StubMe;
impl StubMe {
    pub fn new() -> Self {
        Self
    }

    pub fn do_something(&self) -> i32 {
        // Do something
        13
    }
}

#[test]
fn test_stub() {
    assert_eq!(StubMe::new().do_something(), 13);
}
```

We've returned a different value and tested it, to ensure that we're using the stub version. The real version won't even be compiled in test mode.

> Feature flags let you do this in a more granular manner.

### Mocking


Core Rust doesn't include a mocking system, but there are many available through crates. The `mockall` crate is probably the most widely used.

Let's add `mockall` with `cargo add mockall`.

Mocking is often performed with *traits* rather than concrete structures; if you have the full structure available, it's easy to stub. If you're relying on trait implementations (especially dynamically loaded modules), it's easier to switch out concrete implementations.

Let's start with a pretend trait:

```rust
#[automock]
use mockall::*;

#[automock]
pub trait MyTrait {
    fn calculate(&self, x: u32) -> u32;
}
```

We're not really implying any functionality. We've added the `#[automock]` macro, which creates an implementation named `MockMyTrait` for us---which will by default implement every function included in the trait type.

So now we can write a test that uses the mocked version of `MyTrait`:

```rust
#[test]
fn test_my_mock() {
    // Mockall has added a constructor to build a mocked implementation
    let mut mock = MockMyTrait::new();
    // Tell the mocked trait that `calculate` is expected to return 42
    mock.expect_calculate().return_const(42u32);
    assert_eq!(mock.calculate(12), 42);
}
```

Instead of a constant, you can supply a simple function:

```rust
#[test]
fn test_my_mock_fn() {
    // Mockall has added a constructor to build a mocked implementation
    let mut mock = MockMyTrait::new();
    // Tell the mocked trait that `calculate` is expected to return 42
    mock.expect_calculate().returning(|x| x + 30);
    assert_eq!(mock.calculate(12), 42);
}
```

If you have some pre-defined values to which you know the mock type's answer, you can setup predicates in the mock:

```rust
use mockall::predicate::*;

#[test]
fn test_my_mock_predicate() {
    // Mockall has added a constructor to build a mocked implementation
    let mut mock = MockMyTrait::new();
    // Tell the mocked trait that `calculate` is expected to return 42
    mock.expect_calculate().with(eq(3)).returning(|x| x + 30);
    mock.expect_calculate().with(eq(4)).returning(|x| x + 31);
    assert_eq!(mock.calculate(3), 33);
    assert_eq!(mock.calculate(4), 35);
}
```

> You can, of course, build a function in your test set to return a pre-made mocker.

See the [mockall documentation](https://docs.rs/mockall/latest/mockall/) for an exhaustive list of what it can do. It's an *extensive* list.

## Property Based Testing

An extension of faking, property-based testing feeds large amounts of fake data into a test. Let's build a really simple function:

```rust
pub fn is_email_valid(email: &str) -> bool {
    !email.is_empty() && email.contains('@')
}
```

That's a pretty poor email validator, but it works for illustration.

Now we'll add `fake` to our dev-dependencies:

```toml
[dev-dependencies]
hyper = { version = "0.14", features = ["full"] }
tower = { version = "0.4.13", features = ["util"] }
mime = "0.3"
fake = "2.6"
```

Now we can write a test that uses `fake` to generate an email address:

```rust
#[test]
fn test_random_email() {
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    let email: String = SafeEmail().fake();
    assert!(is_email_valid(&email));
}
```

Wouldn't it be nice to check what's being tested? Let's add a `println!("{email}");` and call our tests with `cargo test -- --nocapture`. In the output, you'll see an email address go by.

That's great for providing fake test-data. How about randomly testing quite a few (property testing)? Let's add another dev dependency, `quickcheck`: `cargo add quickcheck quickcheck_macros`

Now we can write a property-based checker:

```rust
// Both `Clone` and `Debug` are required by `quickcheck`
#[derive(Debug, Clone)]
struct ValidEmailFixture(pub String);

use fake::faker::internet::en::SafeEmail;
use fake::Fake;

impl quickcheck::Arbitrary for ValidEmailFixture {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let email = SafeEmail().fake();
        Self(email)
    }
}

#[quickcheck_macros::quickcheck]
fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
    is_email_valid(&valid_email.0)
}
```

> You can set the `QUICKCHECK_TESTS` environment variable to a number to run the test repeatedly. On Windows, `$env:QUICKCHECK_TESTS = 10`, on *NIX operating systems `export QUICKCHECK_TESTS = 10` or `QUICKCHECK_TESTS = 10 cargo test`