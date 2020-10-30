FROM hasura/graphql-bench:2.0.1-beta

COPY ./queries.graphql /graphql-bench/ws/queries.graphql

EXPOSE 8050
