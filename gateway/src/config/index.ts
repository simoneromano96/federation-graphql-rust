const config = {
  gateway: {
    // logger: {
    //   prettyPrint: TODO: Disabled if prod
    // },
    port: process.env.PORT || 4000,
  },
  services: [
    {
      name: "identity-service",
      url: process.env.IDENTITY_SERVICE_URL || "http://127.0.0.1:4001/graphql",
    },
    {
      name: "products-service",
      url: process.env.PRODUCTS_SERVICE_URL || "http://127.0.0.1:4002/graphql",
    },
  ],
}

export default config
