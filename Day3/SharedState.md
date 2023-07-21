# Dependency Injection & Shared State in Axum Services

In the databases section, we built an SQLite database, setup migrations and uses it to store data. The Tokio + Axum stack fits snugly into an SQLX environment.

## SQLX as a Layer

> The code for this is in `code/axum_sqlx`

Let's build a very simple Axum web service that provides an endpoint that lists all of the messages from our database, in JSON. We'll start by copying over `.env`, and the migrations directory and create the database.

Then we'll add some dependencies:

```toml
[dependencies]
axum = "0.6.19"
serde = { version = "1.0.171", features = ["derive"] }
sqlx = { version = "0.7.1", features = ["sqlite", "runtime-tokio"] }
tokio = { version = "1.29.1", features = ["full"] }
```

We'll start by building our basic Axum program:

```rust
use axum::{routing::get, Router, Json, Extension};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Build a connection pool and run migrations
    let pool = sqlx::SqlitePool::connect("sqlite:hello_db.db").await.unwrap();
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Unable to migrate database");

    // Build a router, and add an extension layer containing the database pool
    let app = Router::new()
        .route("/", get(say_hello_json))
        .layer(Extension(pool));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

There's two new things here: we've copied the database initialization from the previous example, and we've added `.layer(Extension(pool))`. The latter adds the connection pool to the system as a resource layer. This means that any handler that needs the connection pool can get it from the system.

Next, we'll make a structure to model our messages:

```rust
#[derive(Serialize, Deserialize, FromRow)]
struct HelloJson {
    id: i64,
    message: String,
}
```

We're using `Serialize` and `Deserialize` from Serde. We're also using `FromRow` from SQLX. This is a trait that allows SQLX to convert a database row into a structure. We'll use this in a moment.

Finally, we can make the `say_hello_json` handler:

```rust
async fn say_hello_json(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<HelloJson>> {
    let result = sqlx::query_as::<_, HelloJson>("SELECT * FROM messages")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(result)
}
```

Axum uses a clever system of generics to handle arbitrary parameters, so your handlers can specify what they want in the function signature---and Axum/Hyper/Tokio will inject the dependencies. Any extension that requests:

```rust
Extension(pool): Extension<sqlx::SqlitePool>
```

Will receive the connection pool, in a variable named `pool`. So we can then use that to run a query. `query_as` automatically converts the data to our `HelloJson` type. The next bit of magic is that by returning `Json<Vec<HelloJson>>`, Axum will automatically serialize the results into Json (it uses Serde and Serde_json under the hood).

### Sharing the Pool with Other Services

So sharing database connection pools as a resource is easy. But what if we want to share a database connection pool between multiple services? We can do that too. All you need to do is `clone` it. The pool is designed to be cloned (it's similar to `Rc` in how it works), and you can send it off into as many services as you wish to co-locate.

## Shared State

You can share whatever you want with Axum extensions. Whatever you share is subject to the same mutability and protections as regular Rust---so if you need mutability, you need to provide protection.

Let's create a simple cache service that sits in front of our messages table. We'll start by creating a structure:

```rust
use std::collections::HashMap;

struct MessageCache {
    messages: HashMap<i64, HelloJson>
}
```

Then we'll implement some basic cache logic. A real cache would also store negative hits, so you aren't looking them up repeatedly---but this works for illustration:

```rust
impl MessageCache {
    fn new() -> Self {
        MessageCache {
            messages: HashMap::new()
        }
    }

    async fn get(&mut self, id: i64, pool: &sqlx::SqlitePool) -> Option<HelloJson> {
        // Do we have a cached entry?
        if let Some(msg) = self.messages.get(&id) {
            // Yes - return it
            Some(msg.clone())
        } else {
            // No - look it up in the database
            let row = sqlx::query_as::<_, HelloJson>("SELECT * FROM messages WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await;
            if let Ok(row) = row {
                self.messages.insert(row.id, row.clone());
                Some(row)
            } else {
                None
            }
        }
    }
}
```

Now, we can add it to our system:

```rust
let app = Router::new()
    // routes go here
    .layer(Extension(pool))
    .layer(Extension(
        std::sync::Arc::new(
            tokio::sync::Mutex::new(
                MessageCache::new()
            )
        )
    ));
```

I've broken the extension code out into clearly labeled lines. We're using an `Arc` to ensure that there's only one cache, and it is shared between all threads and tasks. Then we protect it with a Tokio `Mutex`---the async version of the same thing from the standard library. Inside that, we create the message cache.

> Normally, you'd save yourself some typing and use `type MessageCache = Arc<Mutex<MessageCache>>;` to make it easier to read.

Now, we can create a handler:

```rust
async fn get_one(
    Path(id): Path<i64>,
    Extension(pool): Extension<sqlx::SqlitePool>,
    Extension(cache): Extension<std::sync::Arc<tokio::sync::Mutex<MessageCache>>>,
) -> Json<HelloJson> {
    let mut lock = cache.lock().await;
    Json(lock.get(id, &pool).await.unwrap())
}
```

We're using an Axum "path extractor" to extract "id" from the path---you'll see that in the route in a moment. Then we request that our `pool` and `cache` extensions be injected. The body just has to lock the cache mutex and perform a lookup.

Finally, we add `one` to the routes:

```rust
let app = Router::new()
        .route("/", get(say_hello_json))
        .route("/one/:id", get(get_one))
```

Running this, we can see that it works. `http://localhost:3000/one/1` returns the first message.