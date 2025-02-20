import { Action, Content, IAgentRuntime, Memory, State } from '@elizaos/core';


export const helloWorldAction: Action = {
  name: 'Hello World',
  description: 'Writes Hello World',
  similes: [],
  examples: [
    [
      {
        user: '{{user1}}',
        content: { text: 'Can you do a Hello World?' }
      },
      {
        user: '{{agentName}}',
        content: {
          text: "I'll execute that for you!",
          action: 'HELLO_WORLD'
        }
      }
    ],
    [
      {
        user: '{{user1}}',
        content: { text: 'Give me a hello world' }
      },
      {
        user: '{{agentName}}',
        content: {
          text: 'Coming right up!',
          action: 'HELLO_WORLD'
        }
      }
    ],
    [
      {
        user: '{{user1}}',
        content: { text: 'Hello World me' }
      },
      {
        user: '{{agentName}}',
        content: {
          text: 'Hello World!',
          action: 'HELLO_WORLD'
        }
      }
    ]
  ],
  validate: async (runtime: IAgentRuntime, message: Memory, state?: State): Promise<boolean> => {
    try {
      const content = message.content;
      if (typeof content.text !== 'string') {
        return false;
      }
      return true;
    } catch {
      return false;
    }
  },
  handler: async (runtime: IAgentRuntime, message: Memory, state?: State): Promise<string> => {
    try {
      return 'Hello World!';
    } catch (error) {
      return 'Hello World failed!';
    }
  }
};
