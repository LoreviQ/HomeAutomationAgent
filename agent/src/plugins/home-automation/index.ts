import { Plugin } from "@elizaos/core";
import { lightOffAction } from "./action";

export const homeAutomationPlugin: Plugin = {
  name: "home-automation",
  description: "Home automation plugin",
  actions: [lightOffAction],
  evaluators: [],
  providers: [],
};
