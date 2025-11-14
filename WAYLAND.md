# Wayland Support in Observer

Observer includes native Wayland support through its use of Tauri 2.x with GTK, which has Wayland compiled in by default.

## What is Wayland?

Wayland is a modern display server protocol for Linux that replaces the older X11 (X Window System). Running applications natively under Wayland provides better security, performance, and modern features compared to running through XWayland (the X11 compatibility layer).

## How Observer Supports Wayland

Observer uses Tauri 2.x with GTK, which is built with Wayland support. When you run Observer on a Wayland session, GTK will automatically use the Wayland backend. No configuration is needed.

### Build-Time Support

Wayland support is compiled into Observer through GTK's dependencies:
- The project's `Cargo.lock` includes `gdkwayland-sys`, `wayland-client`, and related Wayland libraries
- GTK automatically detects whether it's running under Wayland or X11 and uses the appropriate backend
- No special build flags or environment variables are required

### Automatic Backend Selection

GTK applications (including Observer) automatically choose the best backend:
1. On Wayland sessions (`XDG_SESSION_TYPE=wayland`), GTK uses native Wayland
2. On X11 sessions, GTK uses X11
3. No manual configuration is needed

## Verifying Wayland Usage

To verify that Observer is running under native Wayland (not XWayland):

1. **Using `xlsclients`**: If Observer is NOT listed in the output of `xlsclients`, it's running natively under Wayland.

2. **Using `xprop`**: Try clicking on the Observer window with `xprop` running. If it says "no protocol specified" or fails to get properties, Observer is running under Wayland.

3. **Check your session type**: Run `echo $XDG_SESSION_TYPE` - if it returns `wayland`, and Observer works normally, it's using Wayland.

4. **Check with your compositor**: Many Wayland compositors (like Sway or GNOME Shell) can show whether an application is using XWayland or native Wayland in their window information.

## Manual Backend Override (Advanced)

While not recommended for normal use, you can force a specific backend if needed:

```bash
# Force Wayland (only if not auto-detected)
GDK_BACKEND=wayland ./observer

# Force X11 (for debugging)
GDK_BACKEND=x11 ./observer
```

## Building Observer

To build Observer with Wayland support:

```bash
cd app
npm install
npm run tauri build -- --bundles deb
```

The resulting build will automatically include Wayland support through GTK's dependencies. No special configuration is needed.

## Troubleshooting

### Application doesn't start on Wayland

If Observer fails to start on a Wayland session:

1. Verify Wayland session: `echo $XDG_SESSION_TYPE` should return `wayland`
2. Check GTK libraries are installed: `libgtk-3-0` or newer
3. Check the logs for GTK-related errors
4. Try running on X11 to isolate if it's a Wayland-specific issue

### Screen capture issues

Screen capture under Wayland requires additional permissions through the XDG Desktop Portal. Observer handles this automatically through Tauri's built-in support. If you encounter issues:

1. Ensure `xdg-desktop-portal` is installed
2. Ensure your desktop environment's portal backend is installed (e.g., `xdg-desktop-portal-gnome`, `xdg-desktop-portal-kde`)

## Benefits of Native Wayland

- **Better security**: Wayland's security model prevents applications from accessing other windows' content without explicit permission
- **Improved performance**: Direct rendering without X11 overhead
- **Better HiDPI support**: Wayland handles high-resolution displays more elegantly
- **Modern features**: Access to features like screen recording through portals
- **Future-proof**: Wayland is the future of Linux desktop graphics

## Technical Details

Observer's Wayland support comes from:
- **Tauri 2.x**: Uses GTK for Linux UI
- **GTK 3.x**: Compiled with both X11 and Wayland backends
- **Auto-detection**: GTK automatically selects the appropriate backend based on the session type

No application-level code changes are needed for Wayland support - it's provided by the underlying GTK library.

## More Information

- [Wayland Project](https://wayland.freedesktop.org/)
- [GTK on Wayland](https://wiki.gnome.org/Initiatives/Wayland)
- [Tauri Documentation](https://tauri.app/)
