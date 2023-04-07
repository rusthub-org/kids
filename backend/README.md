# Backend: graphql servies server

The **backend** for https://kids.kousun.com, a **Graphql Services Server built on Rust Web Stacks**: tide, async-graphql, async-std, mongodb, jsonwebtoken, base64 & pulldown-cmark ...

## Build & run

``` Bash
git clone https://github.com/rusthub-org/kids.kousun.com
cd ./backend
```

Rename file `.env.example` to `.env`, or put the environment variables into a `.env` file:

```
ADDR=127.0.0.1
PORT=8402
LOG_LEVEL=Debug

SITE_KID=kids.kousun.com
SITE_KEY=QiX7Riw8r..... # Replace with your SITE_KEY
CLAIM_EXP=10000000000

GQL_URI=gql
GQL_VER=v1
GIQL_VER=v1i

MONGODB_URI=mongodb://surfer:surfer@127.0.0.1:27017
MONGODB_NAME=kids
PAGE_SIZE=10
```

Then, build & run:

``` Bash
cargo build
cargo run # or cargo watch -x run
```

GraphiQL: connect to http://127.0.0.1:8402/gql/v1i with browser.
