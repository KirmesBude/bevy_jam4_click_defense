# A Bevy game for bevy jam 4

# How to play
* Start the native app: `cargo run`
* Start the web build: `trunk serve`
    * requires [trunk]: `cargo install --locked trunk`
    * requires `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
    * this will serve your app on `8080` and automatically rebuild + reload it after code changes

You should keep the `credits` directory up to date. The release workflow automatically includes the directory in every build.

### Deploy web build to GitHub pages
 1. Trigger the `deploy-github-page` workflow
 2. Activate [GitHub pages](https://pages.github.com/) for your repository
     1. Source from the `gh-pages` branch (created by the just executed action)
 3. After a few minutes your game is live at `http://username.github.io/repository`

To deploy newer versions, just run the `deploy-github-page` workflow again.

Note that this does a `cargo build` and thus does not work with local dependencies. Consider pushing your "custom Bevy fork" to GitHub and using it as a git dependency.

# Known issues

Audio in web-builds can have issues in some browsers. This seems to be a general performance issue and not due to the audio itself (see [bevy_kira_audio/#9][firefox-sound-issue]).

# License

bevy_jam4_click_defense is free, open source and permissively licensed!
Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!

Some of the code was adapted from other sources.
A lot of it comes from [bevy_game_template](https://github.com/NiklasEi/bevy_game_template).
See [CREDITS.md](CREDITS.md) for the details of the origin of non-original code and assets and licenses of those files.
