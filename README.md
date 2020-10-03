# federation-graphql-rust

Technologies used:

## Rust Identity & Access Management

### Requirements

This service must implement: login; signup; logout; user-info; is-authorized endpoints.

This service must be a REST service and should be a GraphQL service.

Routes details:

* /signup must register a new user into the db

* /login must accept user credentials and must set a session cookie

* /logout must destroy the user session

* /user-info must give the user identity

* /is-authorized must say if a user (can also be a guest) can do something to some resources

### Used libraries

* actix-web

* actix-redis

* actix-session

* rust-argon2

* wither

## Rust GraphQL services

* actix-web

* actix-redis

* actix-session

* async-graphql

* paseto (TBH shouldn't be necessary)

* wither

## Node GraphQL gateway

* fastify

* mercurius 

## Services

The services are:

* accounts

* products

* reviews

