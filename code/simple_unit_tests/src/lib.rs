pub fn double_overflow(n: i32) -> i32 {
    n.overflowing_mul(2).0
}

pub fn double_safe(n: i32) -> Result<i32, String> {
    let (result, overflow) = n.overflowing_mul(2);
    if overflow {
        Err("overflow".to_string())
    } else {
        Ok(result)
    }
}

pub async fn async_double(n: i32) -> i32 {
    n * 2
}

use axum::{
    routing::{get, post},
    Json, Router,
};

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

#[cfg(not(test))]
pub fn complex_math() -> i32 {
    4 * 3 // Let's pretend that's complex
}

#[cfg(test)]
pub fn complex_math() -> i32 {
    12
}

#[cfg(not(test))]
mod stubbing;

#[cfg(not(test))]
pub use stubbing::StubMe;

use mockall::*;

#[automock]
pub trait MyTrait {
    fn calculate(&self, x: u32) -> u32;
}

pub fn is_email_valid(email: &str) -> bool {
    !email.is_empty() && email.contains('@')
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

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("Oops");
    }

    #[test]
    #[ignore]
    fn test_overflow_detection() -> Result<(), String> {
        double_safe(i32::MAX)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_async_double() {
        assert_eq!(4, async_double(2).await);
    }

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

    use axum::{
        body::Body,
        http::{ self, Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot` and `ready`

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

    #[test]
    fn test_my_mock() {
        // Mockall has added a constructor to build a mocked implementation
        let mut mock = MockMyTrait::new();
        // Tell the mocked trait that `calculate` is expected to return 42
        mock.expect_calculate().return_const(42u32);
        assert_eq!(mock.calculate(12), 42);
    }

    #[test]
    fn test_my_mock_fn() {
        // Mockall has added a constructor to build a mocked implementation
        let mut mock = MockMyTrait::new();
        // Tell the mocked trait that `calculate` is expected to return 42
        mock.expect_calculate().returning(|x| x + 30);
        assert_eq!(mock.calculate(12), 42);
    }

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

    #[test]
    fn test_random_email() {
        use fake::faker::internet::en::SafeEmail;
        use fake::Fake;

        let email: String = SafeEmail().fake();
        println!("{email}");
        assert!(is_email_valid(&email));
    }

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
}

