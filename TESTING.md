# Testing Observer Before Deployment

This guide explains how to build and test Observer to ensure everything works properly.

## Prerequisites

Before building, ensure you have:

1. **Node.js** (v16 or later)
2. **Rust** (latest stable)
3. **System dependencies** for Tauri on Linux:
   ```bash
   # Debian/Ubuntu
   sudo apt update
   sudo apt install libwebkit2gtk-4.1-dev \
     build-essential \
     curl \
     wget \
     file \
     libxdo-dev \
     libssl-dev \
     libayatana-appindicator3-dev \
     librsvg2-dev
   ```

## Building Observer

### 1. Clone and Setup

```bash
git clone https://github.com/flooryyyy/Observer.git
cd Observer/app
npm install
```

### 2. Build the Application

```bash
# Build for development (faster, for testing)
npm run tauri dev

# Build for production (.deb package)
npm run tauri build -- --bundles deb
```

The production build will create a `.deb` file in:
```
app/src-tauri/target/release/bundle/deb/
```

### 3. Install and Test the .deb Package

```bash
# Install the package
sudo dpkg -i app/src-tauri/target/release/bundle/deb/observer_*.deb

# If there are dependency issues
sudo apt-get install -f

# Run Observer
observer
```

## Testing Checklist

### 1. Basic Functionality
- [ ] Application starts without errors
- [ ] Main window appears and is responsive
- [ ] Can navigate between different sections

### 2. OpenAI-Compatible Endpoint Support
- [ ] Click the Server icon (top-right corner)
- [ ] Connection settings modal opens
- [ ] "Custom OpenAI-Compatible API Servers" section is visible
- [ ] Can add a custom server URL
- [ ] Can enable/disable servers
- [ ] Can check server status (refresh button)
- [ ] Server status shows "Online" for running servers
- [ ] Models from enabled servers appear in model selector

**Test with a real server:**
```bash
# Start Ollama (if installed)
ollama serve

# Or start another OpenAI-compatible server
# Then add http://localhost:11434 in Observer
```

### 3. Wayland Support (Linux only)

**Check if running on Wayland:**
```bash
# Verify your session type
echo $XDG_SESSION_TYPE
# Should return: wayland

# Launch Observer, then check if it's using Wayland
xlsclients | grep -i observer
# Observer should NOT appear in the list (means it's using Wayland)

# Alternative check with xprop
xprop
# Click on Observer window - should fail or show "no protocol specified"
```

**Check GTK backend (advanced):**
```bash
# Run with debug output
GDK_DEBUG=all observer 2>&1 | grep -i wayland
# Should show Wayland-related messages
```

### 4. Screen Capture (Critical for Observer)
- [ ] Click the eye logo to initialize screen capture
- [ ] Permission dialog appears (on Wayland via portal)
- [ ] Can grant screen capture permission
- [ ] Screen capture works in agents

### 5. Agent Functionality
- [ ] Can create a new agent
- [ ] Can configure agent settings
- [ ] Agent can connect to a model from custom server
- [ ] Agent can run and produce output

### 6. Performance
- [ ] Application launches quickly (< 5 seconds)
- [ ] UI is responsive
- [ ] No memory leaks over time
- [ ] CPU usage is reasonable when idle

## Troubleshooting Build Issues

### Node modules not found
```bash
cd app
npm clean-install
```

### Rust compilation errors
```bash
# Update Rust
rustup update

# Clean build cache
cd app/src-tauri
cargo clean
cd ../..
npm run tauri build
```

### Missing system libraries
```bash
# Ubuntu/Debian - install all dependencies
sudo apt-get install libwebkit2gtk-4.1-dev \
  build-essential curl wget file \
  libxdo-dev libssl-dev \
  libayatana-appindicator3-dev librsvg2-dev

# For Wayland support specifically
sudo apt-get install libgtk-3-dev
```

### .deb package installation fails
```bash
# Check for dependency issues
sudo dpkg -i observer_*.deb
# Note any missing dependencies

# Fix dependencies
sudo apt-get install -f
```

## Automated Testing

Currently, Observer doesn't have automated tests for the UI changes. Testing is manual:

1. Build the application
2. Install and run it
3. Follow the testing checklist above
4. Verify functionality works as expected

## CI/CD Considerations

For continuous integration:

```bash
# In CI environment
cd app
npm ci  # Faster, deterministic install
npm run build  # Build frontend
npm run tauri build -- --bundles deb  # Build package
```

## Verification Commands

Quick commands to verify the build:

```bash
# Check the binary exists
ls -lh app/src-tauri/target/release/app

# Check the .deb package was created
ls -lh app/src-tauri/target/release/bundle/deb/

# Check package contents
dpkg -c app/src-tauri/target/release/bundle/deb/observer_*.deb

# Check desktop entry is included
dpkg -c app/src-tauri/target/release/bundle/deb/observer_*.deb | grep desktop
```

## Testing on Different Environments

### Test on X11
```bash
# If you're on Wayland but want to test X11
GDK_BACKEND=x11 observer
```

### Test on Wayland
```bash
# If you're on X11 but want to test Wayland (requires Wayland session)
# Switch to a Wayland session, then run:
observer
```

### Test with Different Wayland Compositors
- **GNOME Wayland**: Most common, good compatibility
- **Sway**: Minimal compositor, good for testing core functionality
- **KDE Plasma Wayland**: Tests KDE integration

## Logging and Debugging

Enable verbose logging:
```bash
# Tauri debug mode
RUST_LOG=debug observer

# GTK debug
GTK_DEBUG=interactive observer

# Combined
RUST_LOG=debug GTK_DEBUG=interactive observer
```

## Expected Output

When everything is working correctly:

1. **Build output**: Shows successful compilation, no errors
2. **Application launch**: Window appears within 5 seconds
3. **Wayland check**: `xlsclients` doesn't list Observer
4. **Custom servers**: Can add, enable, and connect to servers
5. **Models**: Models from enabled servers appear in dropdowns

## Reporting Issues

If you find issues during testing:

1. Note the exact steps to reproduce
2. Include system information:
   ```bash
   uname -a
   echo $XDG_SESSION_TYPE
   gtk-launch --version
   ```
3. Include relevant logs
4. Describe expected vs actual behavior

## More Information

- **WAYLAND.md**: Details about Wayland support
- **OPENAI_ENDPOINTS.md**: Guide for using custom endpoints
- **README.md**: General project documentation
