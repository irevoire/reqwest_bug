# Reproduce a bug in reqwest

## Step 1
```
cargo run --release --bin server
```

## Step 2
```
cargo run --release --bin reqwest_async
```

On my computer it stops after 16366 requests.

## Step 3
Close the server and do it again with `reqwest_sync`.
