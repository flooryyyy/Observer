# Using OpenAI-Compatible API Endpoints in Observer

Observer supports connecting to any inference server that implements the OpenAI API format. This guide explains how to add and use different OpenAI-compatible endpoints.

## Supported Endpoints

Observer works with any server that provides a `v1/chat/completions` endpoint compatible with the OpenAI API format, including:

- **Ollama** - Local LLM server (http://localhost:11434)
- **llama.cpp server** - Direct model inference
- **vLLM** - High-performance LLM serving
- **LM Studio** - Desktop LLM application
- **llama-swap** - Model switching service
- **LocalAI** - OpenAI alternative
- **text-generation-webui** - With API mode enabled
- **Any custom OpenAI-compatible service**

## How to Add a Custom Server

### Via the UI

1. Click the **Server** icon (🖥️) in the top-right corner of the application
2. Scroll to the **Custom OpenAI-Compatible API Servers** section
3. Click **Add Custom Server**
4. Enter your server URL (e.g., `http://localhost:8080`)
5. Click **Add**
6. Toggle the switch to enable the server
7. Click the refresh icon to check the server status

### Server URL Format

Your server URL should:
- Include the protocol (`http://` or `https://`)
- Point to the base URL (not including `/v1/chat/completions`)
- Be accessible from your local machine

**Examples:**
```
http://localhost:11434          # Ollama default
http://localhost:8080           # LM Studio or custom
http://192.168.1.100:5000       # Network server
https://api.example.com         # Remote HTTPS server
```

## Server Requirements

Your server must implement these OpenAI API endpoints:

### Required: List Models
```
GET /v1/models
```
Returns available models in OpenAI format:
```json
{
  "data": [
    {
      "id": "model-name",
      "object": "model"
    }
  ]
}
```

### Required: Chat Completions
```
POST /v1/chat/completions
```
Accepts OpenAI-compatible chat completion requests and returns streaming or non-streaming responses.

## CORS Considerations

When running Observer as a web application (not the desktop app), you may encounter CORS (Cross-Origin Resource Sharing) issues when connecting to local servers.

### Solutions:

1. **Use the Desktop App (Recommended)**: The Observer desktop app includes a proxy server that handles CORS automatically.

2. **Enable CORS on your server**: Configure your inference server to allow CORS requests from your Observer installation.

3. **Use a reverse proxy**: Set up a reverse proxy (like nginx or Caddy) to handle CORS headers.

## Example: Setting up Ollama

1. Install Ollama: https://ollama.ai
2. Start Ollama (it runs on http://localhost:11434 by default)
3. The local server at http://localhost:3838 should automatically detect it

## Example: Setting up llama-swap

1. Start your llama-swap server on a specific port
2. In Observer, add the custom server URL (e.g., `http://localhost:8000`)
3. Enable the server using the toggle
4. Check the server status with the refresh button
5. Your llama-swap models will now appear in the model selector

## Troubleshooting

### Server shows as "Offline"
- Verify the server is running
- Check the URL is correct (include http:// or https://)
- Ensure the server implements `/v1/models` endpoint
- Check firewall settings if using a network address

### Models not appearing
- Click the refresh button next to the server status
- Verify the server returns models in the correct format
- Check the browser console for error messages

### CORS errors (web version only)
- Use the desktop app which includes CORS handling
- Configure CORS headers on your server
- Use a reverse proxy

## Advanced: Multiple Servers

Observer can connect to multiple inference servers simultaneously:

1. Add all your servers in the settings
2. Enable the ones you want to use
3. Models from all enabled servers will appear in the model selector
4. Each model shows which server it comes from

This allows you to:
- Use different servers for different models
- Switch between local and remote inference
- Balance load across multiple servers
- Compare model performance across providers

## Security Notes

- **Local servers**: When connecting to localhost, ensure only trusted applications are running on those ports
- **Network servers**: Use HTTPS when connecting to remote servers
- **Authentication**: If your server requires authentication, you'll need to configure it to accept requests from Observer's proxy (desktop app) or from your browser origin (web app)

## More Information

- [README.md](README.md) - General Observer documentation
- [WAYLAND.md](WAYLAND.md) - Information about Wayland support
- [OpenAI API Documentation](https://platform.openai.com/docs/api-reference) - API format reference
