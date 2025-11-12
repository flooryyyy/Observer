# Quick Start: Using llama-swap with Observer

This guide shows you how to quickly connect Observer to llama-swap or any other OpenAI-compatible API endpoint.

## Step 1: Start Your API Server

Make sure your API server (llama-swap, Ollama, etc.) is running and accessible. For example:

```bash
# llama-swap example (adjust to your setup)
llama-swap --port 8080

# Or Ollama example
ollama serve  # Runs on port 11434 by default
```

## Step 2: Open Observer Connection Settings

1. Launch the Observer application
2. Click the **Server** icon (🖥️) in the top-right corner of the window

## Step 3: Add Your Custom Server

1. Scroll down to **Custom OpenAI-Compatible API Servers** section
2. Click the **Add Custom Server** button (dashed border)
3. Enter your server URL:
   - For llama-swap: `http://localhost:8080` (or your configured port)
   - For Ollama: `http://localhost:11434`
   - For network servers: `http://192.168.1.100:5000` (example)

4. Click **Add**

## Step 4: Enable the Server

1. Find your newly added server in the list
2. Toggle the switch to **enable** it (switch turns blue)
3. Click the **refresh** icon to check the server status
4. Status should change from "Unchecked" to "Online"

## Step 5: Select a Model

1. Close the connection settings modal
2. Navigate to the agent creation/editing page
3. In the model selector dropdown, you'll now see models from your custom server
4. The server URL is shown next to each model name
5. Select the model you want to use

## Troubleshooting

### Server Shows as "Offline"
- ✓ Verify your server is actually running
- ✓ Check the URL is correct (include `http://` or `https://`)
- ✓ Try accessing `http://your-server:port/v1/models` in a browser
- ✓ Check firewall settings if using a network address

### No Models Appearing
- ✓ Click the refresh button next to the server
- ✓ Ensure your server implements the `/v1/models` endpoint
- ✓ Check the browser console (F12) for error messages

### CORS Errors (Web Version Only)
- **Solution**: Use the desktop app which includes automatic CORS handling
- Alternative: Configure CORS headers on your server
- Alternative: Use a reverse proxy like nginx

## Using Multiple Servers

You can add multiple servers and enable/disable them as needed:

```
✓ Ollama (localhost:11434)     [Enabled]  🟢 Online
✓ llama-swap (localhost:8080)  [Enabled]  🟢 Online  
✓ vLLM (192.168.1.10:8000)    [Disabled] 🟠 Unchecked
```

Models from all **enabled** servers will appear in your model selector.

## Linux: Wayland Support

Observer automatically uses native Wayland on Linux when running in a Wayland session. No configuration is needed.

To verify Observer is using Wayland (not XWayland):
```bash
# Observer should NOT appear here (means it's using Wayland):
xlsclients
```

GTK (which Tauri uses) automatically detects your session type and uses the appropriate backend.

## More Information

- **Detailed Endpoint Guide**: See [OPENAI_ENDPOINTS.md](OPENAI_ENDPOINTS.md)
- **Wayland Support**: See [WAYLAND.md](WAYLAND.md)
- **General Documentation**: See [README.md](README.md)

## Common API Endpoints by Port

| Service | Default Port | URL |
|---------|-------------|-----|
| Ollama | 11434 | http://localhost:11434 |
| LM Studio | 1234 | http://localhost:1234 |
| llama.cpp | 8080 | http://localhost:8080 |
| vLLM | 8000 | http://localhost:8000 |
| text-generation-webui | 5000 | http://localhost:5000 |

(Adjust based on your actual configuration)

---

That's it! You're now connected to your OpenAI-compatible API endpoint. Happy observing! 👁️
