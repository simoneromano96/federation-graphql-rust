import fastify from "fastify"
import mercurius from "mercurius"
import { stitchSchemas } from "@graphql-tools/stitch"
import { loadSchema } from "@graphql-tools/load"
import WebSocket from "ws"
import { UrlLoader } from "@graphql-tools/url-loader"
import config from "./config"

interface Service {
  name: string
  url: string
}

const loadSchemas = (services: Service[]) =>
  services.map(({ url }) =>
    loadSchema(url, {
      enableSubscriptions: true,
      loaders: [new UrlLoader()],
      webSocketImpl: WebSocket,
    }),
  )

// Run the server!
const main = async () => {
  try {
    const app = fastify({
      logger: {
        prettyPrint: true,
      },
    })

    /*
    const schema1 = await loadSchema("http://localhost:4002/graphql", {
      // load from endpoint
      loaders: [new UrlLoader()],
      enableSubscriptions: true,
      webSocketImpl: WebSocket,
    })
    */

    const subschemas = await Promise.all(loadSchemas(config.services))

    console.log(subschemas)

    const gatewaySchema = stitchSchemas({
      subschemas,
    })

    console.log(gatewaySchema)

    app.register(mercurius, {
      schema: gatewaySchema,
      subscription: true,
    })

    await app.listen(config.gateway.port)

    app.log.info(`server listening on ?`, app.server.address())
  } catch (err) {
    // app.log.error(err)
    process.exit(1)
  }
}

main()
