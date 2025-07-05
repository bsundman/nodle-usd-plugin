# Nodle USD Plugin

Universal Scene Description (USD) plugin for the Nodle node editor. This plugin provides USD-specific nodes for creating materials, shaders, and textures in 3D workflows.

## Features

This plugin includes three essential USD shading nodes:

### USD Material Node
- **Category**: 3D > USD > Shading > Materials
- **Description**: Creates a USD material for surface shading
- **Inputs**: Stage, Parent Path, Name, Surface Shader
- **Outputs**: Material Path, Material Reference, Surface Output

### USD Preview Surface Node
- **Category**: 3D > USD > Shading > Shaders
- **Description**: Standard USD preview surface shader with PBR properties
- **Parameters**: Diffuse Color, Metallic, Roughness
- **Outputs**: Surface shader for material connection

### USD Texture Node
- **Category**: 3D > USD > Shading > Textures
- **Description**: USD texture reader for image-based texturing
- **Inputs**: File Path, UV coordinates
- **Outputs**: RGB color, Alpha channel

## Building the Plugin

```bash
cargo build --release
```

The plugin will be built as a dynamic library:
- **Linux**: `target/release/libnodle_usd_plugin.so`
- **macOS**: `target/release/libnodle_usd_plugin.dylib`
- **Windows**: `target/release/nodle_usd_plugin.dll`

## Installing the Plugin

Copy the built library to Nodle's plugins directory:
```bash
# macOS/Linux
cp target/release/libnodle_usd_plugin.dylib ~/.nodle/plugins/

# Or to local plugins folder
cp target/release/libnodle_usd_plugin.dylib ./plugins/
```

## Usage

Once installed, the USD nodes will appear in the node menu under:
- **3D > USD > Shading > Materials** - USD Material
- **3D > USD > Shading > Shaders** - USD Preview Surface
- **3D > USD > Shading > Textures** - USD Texture

These nodes work together to create USD-compliant shading networks:
1. Create a USD Material node
2. Connect a USD Preview Surface to the material's Surface Shader input
3. Connect USD Texture nodes to the preview surface for texturing

## Development

This plugin demonstrates:
- Clean separation of USD functionality from core
- Proper plugin architecture using nodle-plugin-sdk
- Type-safe node implementation
- Integration with egui for parameter UI

## Future Enhancements

This is a minimal USD plugin. Future versions could include:
- Full USD stage management nodes
- Geometry creation nodes (Sphere, Cube, Mesh, etc.)
- Lighting nodes (Distant, Rect, Sphere lights)
- Transform and animation nodes
- USD composition nodes
- Python-based USD engine integration

## License

MIT License - see [LICENSE](LICENSE) for details.