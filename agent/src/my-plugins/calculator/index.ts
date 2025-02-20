import { Plugin } from "@elizaos/core";
import { calculateAction } from "./action.ts";
import { calculateEvaluator } from "./evaluator.ts";

export const calculatorPlugin: Plugin = {
  name: "calculator",
  description: "Basic arithmetic calculator plugin",
  actions: [calculateAction],
  evaluators: [calculateEvaluator],
  providers: [],
};
