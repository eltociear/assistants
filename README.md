
<p align="center">
<img width="600" alt="assistants" src="https://github.com/stellar-amenities/assistants/assets/25003283/d160b5b6-450b-469f-ba6b-929de2e87bcf">
  <h1 align="center">⭐️ Open Source ⭐️ <s>OpenAI</s> Assistants API</h1>

  <h3 align="center">The ⭐️ Open Source ⭐️ <s>OpenAI</s> Assistants API allows you to build AI assistants within your own applications with your own models</h3>

  <p align="center">
    <div align="center">
      <a href="https://discord.gg/XMetBW3zCG"><img alt="Discord" src="https://img.shields.io/discord/1066022656845025310?color=black&style=for-the-badge"></a>
      <hr />
      <a href="https://cal.com/louis030195/unleash-llms">📞 Commercial support</a>
      <br />
      <a href="https://link.excalidraw.com/readonly/YSE7DNzB2LmEPfVdCqq3">🖼️ How does it work?</a>
      <br />
      <a href="https://github.com/stellar-amenities/assistants/issues/new?assignees=&labels=enhancement">✨ Request Feature</a>
      <br />
      <a href="https://github.com/stellar-amenities/assistants/issues/new?assignees=&labels=bug">❤️‍🩹 Report Bug</a>
    </div>
    <br />
  </p>
</p>


# Open Source Assistants API

```ts
const assistant = await openai.beta.assistants.create({
  instructions: "You are a weather bot. Use the provided functions to answer questions.",
  model: "Intel/neural-chat-7b-v3-2",
  tools: [{
    "type": "function",
    "function": {
      "name": "getCurrentWeather",
      "description": "Get the weather in location",
      "parameters": {
          "type": "object",
          "properties": {
          "location": {"type": "string", "description": "The city and state e.g. San Francisco, CA"},
          "unit": {"type": "string"}
          },
          "required": ["location"]
      }
    }
  }]
});
```

[👉 Try it now on your computer](./examples/hello-world-anthropic-curl/README.md).

## News

- [2023/08/12] 🔥 We released an example of an **open source LLM with function calling**. Read the [example](./examples/hello-world-intel-neural-chat-nodejs-function-calling/README.md).
- [2023/29/11] 🔥 We released an example of using **mistral-7b**, an open source LLM. Read the [example](./examples/hello-world-mistral-curl/README.md).

## Overview
The Open Source Assistants API enables building AI assistants within applications using **Open Source** models or **other AI providers than OpenAI**, tools, and knowledge to respond to user queries. This API is in beta, with continuous enhancements and support for various tools.

### Key Features
- [ ] **Code Interpreter**: Runs Python code in a sandboxed environment.
  - [ ] Anthropic
  - [ ] Open source LLMs
- [x] **Knowledge Retrieval**: Retrieves external knowledge or documents.
  - [x] Anthropic
  - [x] Open source LLMs
- [x] **Function Calling**: Defines and executes custom functions.
  - [x] Anthropic
  - [x] Open source LLMs
- [x] **File Handling**: Supports a range of file formats.
  - [x] pdf
  - [x] text files
  - [ ] images, videos, etc.

## Assistants API Beta
- The Assistants API allows integration of AI assistants into applications.
- Supports tools like Code Interpreter, Retrieval, and Function calling.
- Will follow OpenAI Assistants evolutions

## Goals 
- **Highly reliable**: The API is designed to be highly reliable, tested, and used in production.
- **Edge compatible**: Can be used without internet access (on servers, not on consumer hardware)
- **Compatible with OpenAI Assistants API**: The API is designed to be compatible with OpenAI Assistants API.
- **Thin abstraction layer**: The API is designed as a thin, easy to understand, covering most valuable use cases, layer of abstraction on top of the best Open Source projects that have stood the test of time.
- **Free**: The API is designed to be free and Open Source, with no hidden costs.
