import { Action, IAgentRuntime, Memory, State } from '@elizaos/core';


export const lightOffAction: Action = {
    name: 'LIGHT_OFF',
    description: 'Turns the light off',
    similes: [],
    examples: [
        [
            {
                user: '{{user1}}',
                content: { text: 'Turn off the light please' }
            },
            {
                user: '{{agentName}}',
                content: {
                    text: "I'll turn off the light for you!",
                    action: 'LIGHT_OFF'
                }
            }
        ],
        [
            {
                user: '{{user1}}',
                content: { text: 'Could you switch the light off?' }
            },
            {
                user: '{{agentName}}',
                content: {
                    text: 'Of course! Turning off the light now.',
                    action: 'LIGHT_OFF'
                }
            }
        ],
        [
            {
                user: '{{user1}}',
                content: { text: 'Please turn off the lights' }
            },
            {
                user: '{{agentName}}',
                content: {
                    text: 'Sure thing! Switching the lights off.',
                    action: 'LIGHT_OFF'
                }
            }
        ],
    ],
    validate: async (runtime: IAgentRuntime, message: Memory, state?: State): Promise<boolean> => {
        const content = message.content;
        if (typeof content.text !== 'string') {
            return false;
        }
        if (!content.text.toLowerCase().includes('off')) {
            return false;
        }
        return true;
    },
    handler: async (runtime: IAgentRuntime, message: Memory, state?: State): Promise<boolean> => {
        try {
            const response = await fetch('http://127.0.0.1:3030/off');
            return response.status === 200;
        } catch (error) {
            return false;
        }
    }
};
