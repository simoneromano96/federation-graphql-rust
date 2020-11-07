// Require the framework and instantiate it
import fastify from "fastify"
import mercurius from "mercurius"
import { getGraphQLParameters, processRequest } from "graphql-helix"
import { Request as GQLRequest } from "graphql-helix/dist/types"
import fastifyWebsocket from "fastify-websocket"
import { stitchSchemas } from "@graphql-tools/stitch"
import { loadSchema } from "@graphql-tools/load"
import { UrlLoader } from "@graphql-tools/url-loader"

// import { gatewaySchema } from "./graphql/schema"

/*
app.register(fastifyWebsocket)

app.all("/graphql", async (request, response) => {
  const gqlRequest = {
    body: request.body,
    headers: request.headers,
    method: request.method,
    query: request.query,
  }

  const { query, variables, operationName } = getGraphQLParameters(gqlRequest)

  const result = await processRequest({
    schema: gatewaySchema,
    query,
    variables,
    operationName,
    request,
  })

  if (result.type === "RESPONSE") {
    const headers = new Map()
    result.headers.forEach(({ name, value }) => headers.set(name, value))
    response.headers(headers)
    response.status(result.status)
    response.send(result.payload)
  } else {
    // TODO
  }
})

app.get("/ws", { websocket: true }, async (connection, request, params) => {
  connection.socket.on("message", async (message: any) => {
    app.log.info({ message })

    const gqlRequest: GQLRequest = {
      body: message,
      headers: request.headers,
      method: request.method as string,
      query: undefined,
      // query: request.url,
    }

    app.log.info({ gqlRequest })

    const { query, variables, operationName } = getGraphQLParameters(gqlRequest)

    app.log.info(query as string)
    app.log.info(variables as string)
    app.log.info(operationName as string)

    const result = await processRequest({
      schema: gatewaySchema,
      query,
      variables,
      operationName,
      request: { ...request, method: request.method as string, query: undefined },
    })

    app.log.info({ result })

    // message === 'hi from client'
    connection.socket.send("hi from server")
  })
})
*/

// Run the server!
const main = async () => {
  try {
    const app = fastify({ logger: true })

    const schema1 = await loadSchema("http://localhost:4002/graphql", {
      // load from endpoint
      loaders: [new UrlLoader()],
      enableSubscriptions: true,
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
