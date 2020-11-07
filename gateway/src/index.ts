import fastify from "fastify"
import mercurius from "mercurius"
import { stitchSchemas } from "@graphql-tools/stitch"
import { loadSchema } from "@graphql-tools/load"
import WebSocket from "ws"
import { UrlLoader } from "@graphql-tools/url-loader"

// Run the server!
const main = async () => {
  try {
    const app = fastify({ logger: true })

    const schema1 = await loadSchema("http://localhost:4002/graphql", {
      // load from endpoint
      loaders: [new UrlLoader()],
      enableSubscriptions: true,
      webSocketImpl: WebSocket,
    })

    console.log(schema1)

    const gatewaySchema = stitchSchemas({
      subschemas: [schema1],
    })

    console.log(gatewaySchema)

    app.register(mercurius, {
      schema: gatewaySchema,
      subscription: true,
    })

    await app.listen(3000)
    app.log.info(`server listening on ${app.server.address()}`)
  } catch (err) {
    // app.log.error(err)
    process.exit(1)
  }
}

main()
