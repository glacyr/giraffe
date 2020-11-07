# Giraffe

An implementation-agnostic git server.

## Example

1. Run `cargo run --example simple`.
2. Create a new git repository:
    ```sh
    mkdir example && \
        cd example && \
        git init .
    ```
3. Create an empty file and commit it.
    ```sh
    touch README.md && \
        git add . && \
        git commit -m "Initial commit."
    ```
4. Add the `simple`'s git receiver as a remote:
    ```sh
    git remote add origin http://localhost:3030/
    ```
5. Push!
    ```sh
    git push -u origin main
    ```

Expected output:

```diff
  Enumerating objects: 3, done.
  Counting objects: 100% (3/3), done.
  Writing objects: 100% (3/3), 860 bytes | 860.00 KiB/s, done.
  Total 3 (delta 0), reused 0 (delta 0)
+ remote: This is an example message!
  To http://localhost:3030/
   * [new branch]      main -> main
  Branch 'main' set up to track remote branch 'main' from 'origin'.
```

(Note that you can send custom progress messages, like Github does for PRs.)
