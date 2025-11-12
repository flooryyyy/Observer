# Wayland Support in Observer

Observer now includes native Wayland support to ensure optimal performance and compatibility on modern Linux desktop environments.

## What is Wayland?

Wayland is a modern display server protocol for Linux that replaces the older X11 (X Window System). Running applications natively under Wayland provides better security, performance, and modern features compared to running through XWayland (the X11 compatibility layer).

## How Observer Supports Wayland

Observer uses Tauri 2.x with GTK, which has built-in Wayland support. The application is configured to run natively under Wayland when available.

### Desktop Entry Configuration

When installed via the `.deb` package, Observer's desktop entry includes the `GDK_BACKEND=wayland` environment variable, which instructs GTK to use the Wayland backend directly instead of falling back to XWayland.

### Running from Command Line

If you're running Observer directly from the command line, you can ensure it uses Wayland by setting the environment variable:

```bash
GDK_BACKEND=wayland ./observer
```

Or add it to your shell profile:

```bash
export GDK_BACKEND=wayland
```

### Build Script

The included `build.sh` script automatically sets `GDK_BACKEND=wayland` when launching the built application.

## Verifying Wayland Usage

To verify that Observer is running under native Wayland (not XWayland), you can:

1. **Using `xlsclients`**: If Observer is NOT listed in the output of `xlsclients`, it's running natively under Wayland.

2. **Using `xprop`**: Try clicking on the Observer window with `xprop` running. If it says "no protocol specified" or fails to get properties, Observer is running under Wayland.

3. **Check the window title with your compositor**: Many Wayland compositors (like Sway or GNOME Shell) can show whether an application is using XWayland or native Wayland.

## Fallback to X11

If you need to run Observer under X11/XWayland for any reason, you can:

```bash
GDK_BACKEND=x11 ./observer
```

Or simply unset the environment variable before running.

## Troubleshooting

### Application doesn't start under Wayland

If Observer fails to start when `GDK_BACKEND=wayland` is set:

1. Ensure your system has Wayland support: `echo $XDG_SESSION_TYPE` should return `wayland`
2. Check that GTK Wayland libraries are installed: `libgtk-3-0` or `libgtk-4-1`
3. Try running without the environment variable to use the default backend
4. Check the logs for any GTK-related errors

### Screen capture issues

Screen capture under Wayland requires additional permissions through the XDG Desktop Portal. Observer handles this automatically through Tauri's built-in support.

## Benefits of Native Wayland

- **Better security**: Wayland's security model prevents applications from accessing other windows' content without explicit permission
- **Improved performance**: Direct rendering without X11 overhead
- **Better HiDPI support**: Wayland handles high-resolution displays more elegantly
- **Modern features**: Access to features like screen recording through portals
- **Future-proof**: Wayland is the future of Linux desktop graphics

## More Information

- [Wayland Project](https://wayland.freedesktop.org/)
- [GTK on Wayland](https://wiki.gnome.org/Initiatives/Wayland)
- [Tauri Documentation](https://tauri.app/)
