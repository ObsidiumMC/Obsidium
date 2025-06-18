# Server Favicon Setup

This guide explains how to add a custom icon/image to your Minecraft server that will appear in the server list.

## Requirements

- The image must be a **64x64 pixel PNG file**
- **File size limit**: Must be under ~24KB (due to Minecraft protocol string limits)
- PNG should be optimized for small file size
- The image will be displayed in the Minecraft client's server list

## ⚠️ Important Size Limitation

The Minecraft protocol limits strings to 32,767 characters. Since favicons are base64-encoded, your PNG file must be small enough that the encoded result fits within this limit. This typically means:

- **Maximum PNG file size: ~24KB**
- **Recommended size: Under 10KB for safety**

If your favicon is too large, you'll see an error like "String too long" when clients try to connect.

## Method 1: Using a PNG File

1. Create or obtain a 64x64 pixel PNG image for your server icon
2. Save it in your server directory (e.g., `server-icon.png`)
3. Update your server configuration to point to the file:

```rust
let config = ServerConfig::new()
    .with_favicon(Some("server-icon.png".to_string()))
    // ... other config options
```

## Method 2: Using a Base64 Data URL

If you want to embed the icon directly in your code, you can use a base64-encoded data URL:

```rust
let config = ServerConfig::new()
    .with_favicon(Some("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==".to_string()))
    // ... other config options
```

## Method 3: No Favicon

To disable the favicon (no icon will be shown):

```rust
let config = ServerConfig::new()
    .with_favicon(None)
    // ... other config options
```

## Tips

1. **Image Editors**: You can use any image editor that supports PNG export:
   - GIMP (free)
   - Photoshop
   - Paint.NET (Windows)
   - Online tools like Pixlr or Canva

2. **Keep it Simple**: Server icons are displayed very small, so avoid overly complex designs

3. **Test Your Icon**: Add your server to your Minecraft client and check how the icon appears

4. **File Location**: Place the icon file in your server's working directory or use an absolute path

## Troubleshooting

- **"String too long" error**: Your PNG file is too large. The file must be under ~24KB when base64-encoded
- **Icon not appearing**: Check that the file path is correct and the file exists
- **Invalid format error**: Ensure your image is exactly 64x64 pixels and saved as PNG
- **File permission error**: Make sure the server has read access to the image file

### How to Optimize Your PNG for Size

1. **Use fewer colors**: Reduce the color palette in your image editor
2. **Use PNG-8 instead of PNG-24** if possible (8-bit color vs 24-bit)
3. **Use online PNG optimizers**:
   - TinyPNG (https://tinypng.com/)
   - OptiPNG
   - PNG Gauntlet
4. **Keep the design simple**: Complex images with many colors create larger files
5. **Use solid colors and avoid gradients** when possible

### Check Your Favicon Size

You can check if your favicon will work by looking at the file size:
- **Under 10KB**: ✅ Will definitely work
- **10-24KB**: ⚠️ Should work, but optimize if possible  
- **Over 24KB**: ❌ Too large, must be optimized

## Example Icons

Some ideas for server icons:
- Your server's logo
- A themed image related to your server type (medieval, modern, etc.)
- Minecraft blocks or items
- Simple geometric designs
- Community emblems

The favicon will automatically be loaded when the server starts and will be sent to clients when they ping your server for the server list.
