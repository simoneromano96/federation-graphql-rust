const config = {
    gateway: {
        port: process.env.PORT || 4000,
    },
    services: [{
        name: "identity-service",
        url: process.env.IDENTITY_SERVICE_URL || "http://127.0.0.1:4001/graphql",
    }, {
        name: "products-service",
        url: process.env.PRODUCTS_SERVICE_URL || "http://127.0.0.1:4002/graphql",
    }]
};

module.exports = {config};
