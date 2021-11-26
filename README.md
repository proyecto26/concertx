# ConcertX

**ConcertX** exists to help bring unique concerts to life. It's a funding platform where every musician sets their concerts funding goal and deadline.

![Make concerts for everyone from everywhere](https://user-images.githubusercontent.com/2154886/143656674-84bd7a31-c6e0-464c-b532-c31f3906d532.png)


ConcertX seeks help backers pledge to concerts to help them come to life and support a creative process. To thank their backers for their support, musicians offer unique rewards that speak to the spirit of what they're hoping to create.


## Getting Started 🚀

- Empower People, Recording studios, and Music creating **Web 3.0** concerts.
- From your room, from the comfort of your home, from a recording studio; create music to be heard from anywhere.
- Immersion for your audience; provides with VR and 3D cross-platform experiences a unique perception of your music for your fans.
- Be rebellious! 🤘 **Smart contracts** allow trusted transactions directly with your audience without a central authority. 🏛️


To run this project locally:

1. Prerequisites: Make sure you've installed [Node.js] ≥ 12
2. Install dependencies: `yarn install`
3. Run the local development server: `yarn dev` (see `package.json` for a
   full list of `scripts` you can run with `yarn`)

Now you'll have a local development environment backed by the NEAR TestNet!

Go ahead and play with the app and the code. As you make code changes, the app will automatically reload.

### Exploring The Code

1. The "backend" code lives in the `/contract` folder. See the README there for
   more info.
2. The frontend code lives in the `/src` folder. `/src/index.html` is a great
   place to start exploring. Note that it loads in `/src/index.js`, where you
   can learn how the frontend connects to the NEAR blockchain.
3. Tests: there are different kinds of tests for the frontend and the smart
   contract. See `contract/README` for info about how it's tested. The frontend
   code gets tested with [jest]. You can run both of these at once with `yarn
   run test`.

### Deploy

Every smart contract in NEAR has its [own associated account][NEAR accounts]. When you run `yarn dev`, your smart contract gets deployed to the live NEAR TestNet with a throwaway account. When you're ready to make it permanent, here's how.


Step 0: Install near-cli (optional)
-------------------------------------

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `yarn install`, but for best ergonomics you may want to install it globally:

    yarn install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)


Step 1: Create an account for the contract
------------------------------------------

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `ConcertX.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `ConcertX.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

      near login

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):

      near create-account ConcertX.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet


Step 2: set contract name in code
---------------------------------

Modify the line in `src/config.js` that sets the account name of the contract. Set it to the account id you used above.

    const CONTRACT_NAME = process.env.CONTRACT_NAME || 'ConcertX.YOUR-NAME.testnet'


Step 3: deploy!
---------------

One command:

    yarn deploy

As you can see in `package.json`, this does two things:

1. builds & deploys smart contract to NEAR TestNet
2. builds & deploys frontend code to GitHub using [gh-pages]. This will only work if the project already has a repository set up on GitHub. Feel free to modify the `deploy` script in `package.json` to deploy elsewhere.

## Dependencies

- [NEAR](https://docs.near.org/docs/develop/basics/getting-started) - Smart Contracts
- [Rust](https://docs.near.org/docs/develop/contracts/rust/intro#installing-the-rust-toolchain) - `rustup default 1.56.1`
- [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/) - Building, testing, and publishing Rust-generated WebAssembly.
- [cargo](https://doc.rust-lang.org/cargo/) - `cargo install --force cargo-audit --features=fix`
- [wasm32-unknown-unknown](https://github.com/rustwasm/wasm-bindgen/issues/979) - `rustup target add wasm32-unknown-unknown`
- [cargo-generate](https://github.com/cargo-generate/cargo-generate) - `cargo install cargo-generate`
- [BabylonJS React Native](https://www.babylonjs.com/reactnative/) - Babylon React Native combines the React Native framework with the power, beauty, and simplicity of Babylon.js to unlock the ability to create 3D cross-platform experiences.

## Troubleshooting

On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.


  [React]: https://reactjs.org/
  [create-near-app]: https://github.com/near/create-near-app
  [Node.js]: https://nodejs.org/en/download/package-manager/
  [jest]: https://jestjs.io/
  [NEAR accounts]: https://docs.near.org/docs/concepts/account
  [NEAR Wallet]: https://wallet.testnet.near.org/
  [near-cli]: https://github.com/near/near-cli
  [gh-pages]: https://github.com/tschaub/gh-pages

## Supporting 🍻
I believe in Unicorns 🦄
Support [me](http://www.paypal.me/jdnichollsc/2), if you do too.

## Happy coding 💯
Made with ❤️

<img width="150px" src="https://avatars0.githubusercontent.com/u/28855608?s=200&v=4" align="right">
