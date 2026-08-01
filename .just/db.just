# generate db schema and model files based on migration files
[group: 'db']
db-codegen:
  #!/usr/bin/env bash
  set -euo pipefail
  export DATABASE_URL=postgres://$(cat local_secrets/psql_user):$(cat local_secrets/psql_pass)@localhost/logistics
  diesel migration run
  diesel_ext --model \
    --add-table-name \
    --rust_styled_model_fields \
    --import-types "crate::psql::schema::*" \
    --derive "Queryable, Selectable, Insertable, AsChangeset, Debug, Clone, serde::Serialize, core::cmp::Eq, core::cmp::PartialEq, core::hash::Hash" \
    > src/psql/models.rs

# revert all applied diesel migrations
[group: 'db']
db-revert:
  #!/usr/bin/env bash
  set -euo pipefail
  export DATABASE_URL=postgres://$(cat local_secrets/psql_user):$(cat local_secrets/psql_pass)@localhost/logistics
  diesel migration revert

# apply all diesel migrations
[group: 'db']
db-migrate:
  cargo run -- db migrate

# run the database
[group: 'db']
run-db:
  #!/usr/bin/env bash
  if [[ ! -f local_secrets/psql_user ]]; then
    echo "generating postgres credentials"
    mkdir -p local_secrets
    echo -n "logistics" > local_secrets/psql_user
    openssl rand 60 | openssl base64 -A | sed 's|/|+|g' > local_secrets/psql_pass
  fi
  docker stop logistics_psql ||:
  mkdir -p psql/data
  docker run -it --rm --name logistics_psql \
    -p 5432:5432 \
    --mount=type=bind,src=$PWD/psql/data,target=/var/lib/postgresql \
    --mount=type=bind,src=$PWD/local_secrets,target=/secrets \
    --env POSTGRES_PASSWORD_FILE=/secrets/psql_pass \
    --env POSTGRES_USER_FILE=/secrets/psql_user \
    postgres:18.3

# get an interactive shell in the locally running db
[group: 'db']
db-shell:
  docker exec -it logistics_psql bash -c 'psql -U $(cat /secrets/psql_user) logistics'
