const Fastify = require("fastify")
const mercurius = require('mercurius')

const gateway = Fastify()

const main = async () => {
    gateway.register(mercurius, {
        graphiql: "playground",
        subscription: true,
        gateway: {
            services: [
                {
                    name: 'accounts',
                    url: 'http://127.0.0.1:4001/',
                    mandatory: true,
                    // Forward all headers
                    rewriteHeaders: (headers) => headers,
                },
                {
                    name: 'products',
                    url: 'http://127.0.0.1:4002/',
                    // Forward all headers
                    rewriteHeaders: (headers) => headers,
                }
        ]
    },
    pollingInterval: 2000,
    errorHandler: (error, service) => {
        console.error("Service: ", service);
        console.error("Error: ", error);
      },
    })

    await gateway.listen(4000)

    console.log("API Gateway available at http://localhost:4000/graphql")
}

main()

/*
const { ApolloServer } = require("apollo-server");
const { ApolloGateway } = require("@apollo/gateway");
 
const gateway = new ApolloGateway({
  serviceList: [
    { name: "accounts", url: "http://localhost:4001" },
    { name: "products", url: "http://localhost:4002" },
  ]
});
 
const server = new ApolloServer({ gateway, subscriptions: false });
 
server.listen(4000).then(({ url }) => {
  console.log(`🚀 Server ready at ${url}`);
});
*/