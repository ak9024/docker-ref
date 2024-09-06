# docker-ref

A repository for demonstrating Docker reference implementations and usage patterns.

## Docker

- [x] Go (fiber)
- [x] Rust (actix)
- [ ] Node.js
- [ ] Ptyhon (fastAPI)
- [ ] Next.js
- [ ] Deno
- [ ] Bun
- [ ] CRA vite
- [ ] Svelte
- [ ] PHP (Laravel)

## Images

| REPOSITORY | TAG      | SIZE   |
|------------|----------|--------|
| rust-app   | optimize | 84.5MB |
| rust-app   | latest   | 87.6MB |
| go-app     | upx      | 1.93MB |
| go-app     | latest   | 5.99MB |

## How to run?

Please go to `./apps/` then

```bash
# example docker build for go apps
docker build -t go-app:latest -f Dockerfile .
# check images
docker images
```

## Contributing

Contributions are welcome! Please follow the guidelines below:

- Fork the repository.
- Create a new branch (git checkout -b feature-branch).
- Commit your changes (git commit -m 'Add new feature').
- Push the branch (git push origin feature-branch).
- Open a Pull Request.
