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

```bash
REPOSITORY   TAG        IMAGE ID       CREATED        SIZE
rust-app     optimize   6ef45873c2d7   3 hours ago    84.5MB
go-app       upx        d8ac9edc180d   16 hours ago   1.93MB
go-app       latest     4adb1f990dce   16 hours ago   5.99MB
rust-app     latest     171b8bbaf046   16 hours ago   87.6MB
```

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
