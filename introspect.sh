#!/bin/bash

graphql-client introspect-schema --output src/musicbot_admin.json http://musicbot_admin:123_musicbot_admin_321@51.79.24.206:81/graphql
graphql-client generate src/user/queries/user_account_list.graphql --schema-path=src/musicbot_admin.json --variables-derives=Debug --response-derives=Debug -o=src/user/queries

graphql-client introspect-schema --output src/musicbot.json http://51.79.24.206/graphql

graphql-client generate src/user/queries/auth.graphql --schema-path=src/musicbot.json --variables-derives=Debug --response-derives=Debug -o=src/user/queries
graphql-client generate src/user/queries/whoami.graphql --schema-path=src/musicbot.json --variables-derives=Debug --response-derives=Debug -o=src/user/queries
graphql-client generate src/user/queries/register.graphql --schema-path=src/musicbot.json --variables-derives=Debug --response-derives=Debug -o=src/user/queries
graphql-client generate src/user/queries/unregister.graphql --schema-path=src/musicbot.json --variables-derives=Debug --response-derives=Debug -o=src/user/queries
graphql-client generate src/user/queries/current_musicbot.graphql --schema-path=src/musicbot.json --variables-derives=Debug --response-derives=Debug -o=src/user/queries

graphql-client generate src/music/queries/upsert.graphql --schema-path=src/musicbot.json --variables-derives=Debug --response-derives=Debug -o=src/music/queries
graphql-client generate src/music/queries/clean.graphql --schema-path=src/musicbot.json --variables-derives=Debug --response-derives=Debug -o=src/music/queries
graphql-client generate src/music/queries/stats.graphql --schema-path=src/musicbot.json --variables-derives=Debug --response-derives=Debug -o=src/music/queries
graphql-client generate src/music/queries/playlist.graphql --schema-path=src/musicbot.json --variables-derives=Debug --response-derives=Debug -o=src/music/queries


graphql-client generate src/filter/queries/count.graphql --schema-path=src/musicbot.json --variables-derives=Debug --response-derives=Debug -o=src/filter/queries
graphql-client generate src/filter/queries/get.graphql --schema-path=src/musicbot.json --variables-derives=Debug --response-derives=Debug -o=src/filter/queries
graphql-client generate src/filter/queries/list.graphql --schema-path=src/musicbot.json --variables-derives=Debug --response-derives=Debug -o=src/filter/queries
graphql-client generate src/filter/queries/delete.graphql --schema-path=src/musicbot.json --variables-derives=Debug --response-derives=Debug -o=src/filter/queries
graphql-client generate src/filter/queries/upsert.graphql --schema-path=src/musicbot.json --variables-derives=Debug --response-derives=Debug -o=src/filter/queries
