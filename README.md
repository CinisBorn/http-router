# Description 

A router developed for learning purposes.

This router is designed to work with my personal web server. It does not support common features such as pattern matching, 
although I plan to implement this feature in the future.

This project is not production-ready and may currently contain bugs and security vulnerabilities.

# Usage 

Creates a new `Router` instance with `new` associated function, then register endpoints specifying 
a *target*  and a *handle* 

```rust
let mut router = Router::new();

router.get("/test", |_| {
  let mut response = Response::new();
  response.set_status(CodeStatus::Ok);

  response
});

```

Please note `Response` and `CodeStatus` are from `http-core` crate. It's a dependency. You can see more at [http-core](https://github.com/CinisBorn/http-core).
