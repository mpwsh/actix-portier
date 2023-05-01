### Description

Actix web using [Portier](https://portier.github.io) for authentication and Handlebars templates to render HTML.

### Usage

Start redis (for session persistance) using `docker-compose`.

```bash
docker-compose up -d
```

Update [conf/local.yaml](conf/local.yaml) and/or [conf/base.yaml](conf/base.yaml) to suit your needs and start the server.

```bash
cargo run
```

Access the homepage at `127.0.0.1:8000`.

Available endpoints:

- [/](http://127.0.0.1:8000/login)
- [/login](http://127.0.0.1:8000/login)
- [/dashboard](http://127.0.0.1:8000/logout)
- [/logout](http://127.0.0.1:8000/logout) - clears session

### Credits

Most of the code comes from the awesome [zero-to-production](https://github.com/LukeMathWalker/zero-to-production) repository, by [LukeMathWalker](https://twitter.com/algo_luca).

HTML Forms are adapted from this [Minimal Login Form](https://codepen.io/rikosteo/pen/vwrwMe) by [Thodoris Thomaidis](https://codepen.io/rikosteo)
