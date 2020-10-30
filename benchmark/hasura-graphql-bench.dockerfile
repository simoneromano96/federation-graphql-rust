FROM hasura/graphql-bench

COPY ./queries.graphql /graphql-bench/ws/queries.graphql

EXPOSE 8050
