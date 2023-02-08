# Graphql Services Server - tide + async-graphql

## Build & run

``` Bash
git clone https://github.com/piexue/piexue.com
cd piexue.com
cargo build

cd backend
```

Rename file `.env.example` to `.env`, or put the environment variables into a `.env` file:

```
ADDR=127.0.0.1
PORT=8400
LOG_LEVEL=Debug

SITE_KID=piexue.com
SITE_KEY=QiX7Riw8r..... # Replace with your SITE_KEY
CLAIM_EXP=10000000000

GQL_URI=gql
GQL_VER=v1
GIQL_VER=v1i

MONGODB_URI=mongodb://surfer:surfer@127.0.0.1:27017
MONGODB_NAME=jobs
PAGE_SIZE=10
```

Then, build & run:

``` Bash
cargo run
```

or

``` Bash
cargo watch -x run
```

GraphiQL: connect to http://127.0.0.1:8400/gql/v1i with browser.
