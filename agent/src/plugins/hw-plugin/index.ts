import { Plugin } from "@elizaos/core";
import { helloWorldAction } from "./action.ts";

export const helloWorldPlugin: Plugin = {
  name: "hello-world",
  description: "Basic hello world plugin",
  actions: [helloWorldAction],
  evaluators: [],
  providers: [],
};
