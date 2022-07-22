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

## Other included lib

You can do the step 2 with `reqwest_sync` or `ureq`.