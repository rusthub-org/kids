# Web Application Server - handlebars

Demo site: [https://piexue.com](https://piexue.com)

## Build & run

``` Bash
git clone https://github.com/piexue/piexue.com
cd piexue.com
cargo build

cd frontend
```

Rename file `.env.example` to `.env`, or put the environment variables into a `.env` file:

```
DOMAIN=piexue.com
ADDR=127.0.0.1
PORT=7400
LOG_LEVEL=Debug

GQL_PROT=http
GQL_ADDR=127.0.0.1
GQL_PORT=8400
GQL_URI=gql
GQL_VER=v1
GIQL_VER=v1i

EMAIL_SMTP=<smtp.server>
EMAIL_FROM=<email_account>
EMAIL_USERNAME=<username>
EMAIL_PASSWORD=<password>
```

Build & Run:

``` Bash
cargo run
```

or

``` Bash
cargo watch -x run
```
Then connect to http://127.0.0.1:7400 with browser.
