# Complete GraphQL with Apollo Federation services in Rust

## Services

The services are:

* accounts

* products

* reviews

## Rust Identity & Access Management

### Requirements

This service must implement: login; signup; logout; user-info; is-authorized endpoints.

This service must be a REST service and should be a GraphQL service where possible, every service must be documented (REST with OpenAPI and GraphQL with the builtin schema docs).

Routes details:

* /signup must register a new user into the db

* /login must accept user credentials and must set a session cookie

* /user-info must give the user identity

* /logout must destroy the user session

The following routes must be protected:

* /is-authorized must say if a user (can also be a guest) can do something to some resources

* /add-policy must add the policy triplet (subject, action, object)

* /delete-policy must remove the policy triplet (subject, action, object)

* /add-role must add a role

* /remove-role must remove a role

* /add-user-to-role must add a user to a role

* /remove-user-from-role must remove a user from a role

### Libraries

* actix-web

* actix-redis

* actix-session

* rust-argon2

* wither

* paperclip

* casbin

## Rust GraphQL services

### Libraries

* actix-web

* actix-redis

* actix-session

* async-graphql

* paseto (TBH shouldn't be necessary)

* wither

## Node GraphQL gateway

### Libraries

* fastify

* mercurius 

## Decisions and issues

I'd like to have a only GraphQL architecture but there is a problem: I can't handle stateful sessions in graphql resolvers; this would force me to handle sessions in a stateless way, which I personally don't like, cookie sessions are the most secure way to handle the user identity.

I could use Paseto to generate some tokens but this don't solve the main problem of where I should store them.

So the Identity/Accounts Service (they should be merged) exposes both "REST" (I wouldn't call them REST TBH) and GraphQL API standards.

I believe that the tokens are as secure as where they are stored, so having them in the local storage or session storage in a untrusted client is bad, cookies, on the other hand, can be blocked from reading from JS (HTTP Only), are set automatically and they can be invalidated (with a real logout).

Tokens are good/perfect for machine-to-machine communication where the parties are trusted.

When creating an IAM service I believe the hardest decisions are about the DB and the hashing algorithm.

I choose Mongo mostly because it is "cluster-able" easily, this will improve both the performance and the safety of the data at the cost of having some un-synced data, this is not a big deal though because I don't expect a lot of updates on the user model.

Another thing I like about mongo for is how easily I can add/remove data from the collection being a "schema-less" database, I do expect a lot of changes in the User collection as my ecosystem grows.

I know about cockroachDB which is a postgres-compatible scalable DB but for the preceding reason I'm not sure if it is a good choice, it will force me to handle migrations.

Until now I've talked about Authentication mostly, now I should solve the problem of Authorization, I believe that the best Authorization mechanism is the RBAC (Role-Based access control) which defines some roles and actions where each role can execute some determinate actions. 

In my services I have the session, so I can get the current User before resolving the Graph query, but I'm still not sure on how to handle all this, I'd like to keep everything in my IAM service or I could also detatch it with an Access Control and Permission Management service, I'm not sure if detatching is a good idea right now, mostly because the only endpoint I can think of is the "/is-authorized" endpoint, that expects the user role and the action and gives back if he can execute the action, but this means that there will be an added latency that I'm not sure I like, if I could cache the roles and actions tough it might be ok.

The chosen library for the access control is casbin, it is a multi-language library with an adapter for Rust and SQLx, it exposes an admin portal too.

Now there is another issue, I don't like having a service using both mongo and postgres, using both the wither and SQLx dependencies, I think I should merge them into postgres and use SQLx.

It's unclear to me currently how to enable federation in the services, if I put `graphql(entity)` to a query that doesn't have any input (a Read All for example), it says that it must have an input, if it has an input it is not exposed to the gateway, but if there is no `graphql(entity)` in the query schema nothing is exposed to the gateway, I had to put some fake queries/mutations that return the type that is needed for the other queries/mutations.

Turns out I didn't understand at all the entity concept, it is internally used between the services to get particular informations, marking something with entity will not expose it externally but internally between the services, there is no need to put them in mutations too, just in a queries for each entity.

If I need to expose both an entity and a query that fetches that entity the code must be sadly duplicated as for Federation design.

To handle subscriptions, since I already have redis, I will use the pub-sub interface of redis avoiding more useless dependencies.
