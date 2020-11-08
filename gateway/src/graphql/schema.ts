import { makeExecutableSchema } from "@graphql-tools/schema"
import { addMocksToSchema } from "@graphql-tools/mock"
import { stitchSchemas } from "@graphql-tools/stitch"

let chirpSchema = makeExecutableSchema({
  typeDefs: `
    type Chirp {
      id: ID!
      text: String
      authorId: ID!
    }

    type Query {
      chirpById(id: ID!): Chirp
      chirpsByAuthorId(authorId: ID!): [Chirp]!
    }
  `,
})

let authorSchema = makeExecutableSchema({
  typeDefs: `
    type User {
      id: ID!
      email: String
    }

    type Query {
      userById(id: ID!): User
    }
  `,
})

// just mock the schemas for now to make them return dummy data
chirpSchema = addMocksToSchema({ schema: chirpSchema })
authorSchema = addMocksToSchema({ schema: authorSchema })

// setup subschema configurations
export const chirpSubschema = { schema: chirpSchema }
export const authorSubschema = { schema: authorSchema }

// build the combined schema
export const gatewaySchema = stitchSchemas({
  subschemas: [chirpSubschema, authorSubschema],
})
