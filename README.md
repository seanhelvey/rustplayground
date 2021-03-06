# Rust Playground

A Rust back-end built with the [Rocket](https://github.com/SergioBenitez/Rocket) framework.

Heroku deployment notes from [this GitHub issue](https://github.com/SergioBenitez/Rocket/issues/171):

    $ heroku create --buildpack https://github.com/emk/heroku-buildpack-rust.git
    $ git remote add heroku https://git.heroku.com/<heroku-project-name>.git
    $ echo "web: ROCKET_PORT=$PORT ROCKET_ENV=prod ./target/release/<MyApp>" > Procfile
    # Dynamically binds Rocket to the Dyno's $PORT on 0.0.0.0
    $ echo "VERSION=nightly" > RustConfig
    # Ensures the buildpack uses Rust nightly
    $ git add . && git commit -m "Add Heroku deploy configuration"
    $ git push heroku master

Note: When following steps above, just don't forget to replace <MyApp> with your app name!

Rocket needs Rust nightly:
- `rustup update nightly`
- `rustup default nightly`

Running locally:
- `cargo build`
- `cargo run`