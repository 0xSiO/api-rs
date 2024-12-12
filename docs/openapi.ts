import { OpenAPIV3_1 } from "openapi-types";

const doc: OpenAPIV3_1.Document = {
  openapi: "3.1.0",
  info: {
    title: "API Docs",
    version: "0.0.0",
  },
  paths: {},
};

console.log(JSON.stringify(doc));
