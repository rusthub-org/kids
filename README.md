# api.kousun.com

[中文](./README-ZH.md)

The **backend** for https://kousun.com, a **Graphql Services Server built on Rust Web Stacks**: tide, async-graphql, async-std, mongodb, jsonwebtoken, base64 & pulldown-cmark ...

> [KouSun | 蔻隼](https://kousun.com) aims to build a **multi-language** CMS(Content Management System) based on **Rust Web stacks**, with long-term upgrade and maintenance.

## Demo site

- [jobs.kousun.com - Projects Matchmaking](https://jobs.kousun.com)
- [kids.kousun.com - Kids Education](https://kids.kousun.com)
- [niqin.com - Books Platform](https://niqin.com)

## Build & run

``` Bash
git clone https://github.com/rusthub-org/api.kousun.com
cd api.kousun.com
```

Rename file `.env.example` to `.env`, or put the environment variables into a `.env` file:

```
ADDR=127.0.0.1
PORT=8400
LOG_LEVEL=Debug

SITE_KID=api.kousun.com
SITE_KEY=QiX7Riw8r..... # Replace with your SITE_KEY
CLAIM_EXP=10000000000

GQL_URI=gql
GQL_VER=v1
GIQL_VER=v1i

MONGODB_URI=mongodb://surfer:surfer@127.0.0.1:27017
MONGODB_NAME=kousun
PAGE_SIZE=10
```

Then, build & run:

``` Bash
cargo build
cargo run # or cargo watch -x run
```

GraphiQL: connect to http://127.0.0.1:8400/gql/v1i with browser.

## Frontend

- [kousun-jobs](https://github.com/rusthub-org/jobs.kousun.com)
- [kousun-kids](https://github.com/rusthub-org/kids.kousun.com)

See also:

- https://github.com/zzy/tide-async-graphql-mongodb - Clean boilerplate for graphql services, wasm/yew & handlebars frontend. 
- https://github.com/zzy/surfer - Simple WIP blog & upcoming upgrades.
