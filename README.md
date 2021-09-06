graphql-client generate src/queries.graphql --schema-path=src/musicbot.json --variables-derives=Debug -o=src

graphql-client generate src/auth.graphql --schema-path=src/musicbot.json --variables-derives=Debug -o=src
graphql-client generate src/register.graphql --schema-path=src/musicbot.json --variables-derives=Debug -o=src
graphql-client generate src/upsert_music.graphql --schema-path=src/musicbot.json --variables-derives=Debug -o=src
graphql-client generate src/current_musicbot.graphql --schema-path=src/musicbot.json --variables-derives=Debug -o=src

cargo clippy --all-targets --all-features -- -D warnings
