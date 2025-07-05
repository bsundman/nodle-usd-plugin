//! Comprehensive USD Plugin for Nodle
//! 
//! This plugin provides complete Universal Scene Description (USD) functionality.

use nodle_plugin_sdk::*;
use std::collections::HashMap;

// Include core module for USD engine and Python integration
mod core;

// Include viewport module with complete 3D rendering
mod viewport;

// USD Plugin
pub struct USDPlugin;

impl NodePlugin for USDPlugin {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "USD Plugin".to_string(),
            version: "0.2.0".to_string(),
            author: "Nodle Contributors".to_string(),
            description: "Complete Universal Scene Description (USD) support for Nodle".to_string(),
            compatible_version: "0.1.0".to_string(),
        }
    }
    
    fn register_nodes(&self, registry: &mut dyn NodeRegistryTrait) {
        println!("Registering comprehensive USD nodes...");
        
        // Register the USD Viewport node
        let _ = registry.register_node_factory(Box::new(crate::viewport::USDViewport::default()));
        println!("✅ USD Viewport node registered");
        
        // Register Stage nodes
        let _ = registry.register_node_factory(Box::new(USDCreateStageFactory::default()));
        let _ = registry.register_node_factory(Box::new(USDLoadStageFactory::default()));
        let _ = registry.register_node_factory(Box::new(USDSaveStageFactory::default()));
        println!("✅ USD Stage nodes registered");
        
        // Register Geometry nodes
        let _ = registry.register_node_factory(Box::new(USDMeshFactory::default()));
        let _ = registry.register_node_factory(Box::new(USDSphereFactory::default()));
        let _ = registry.register_node_factory(Box::new(USDCubeFactory::default()));
        let _ = registry.register_node_factory(Box::new(USDCylinderFactory::default()));
        println!("✅ USD Geometry nodes registered");
        
        // Register Transform nodes
        let _ = registry.register_node_factory(Box::new(USDXformFactory::default()));
        let _ = registry.register_node_factory(Box::new(USDTranslateFactory::default()));
        let _ = registry.register_node_factory(Box::new(USDRotateFactory::default()));
        let _ = registry.register_node_factory(Box::new(USDScaleFactory::default()));
        println!("✅ USD Transform nodes registered");
        
        // Register Lighting nodes
        let _ = registry.register_node_factory(Box::new(USDDistantLightFactory::default()));
        let _ = registry.register_node_factory(Box::new(USDSphereLightFactory::default()));
        let _ = registry.register_node_factory(Box::new(USDDomeLightFactory::default()));
        println!("✅ USD Lighting nodes registered");
        
        // Register Shading nodes
        let _ = registry.register_node_factory(Box::new(USDMaterialFactory::default()));
        let _ = registry.register_node_factory(Box::new(USDShaderFactory::default()));
        let _ = registry.register_node_factory(Box::new(USDTextureFactory::default()));
        println!("✅ USD Shading nodes registered");
        
        // Register additional viewport nodes
        let _ = registry.register_node_factory(Box::new(USDStageInspectorFactory::default()));
        println!("✅ USD Viewport nodes registered");
        
        println!("🎉 All USD nodes registered successfully!");
    }
    
    fn get_menu_structure(&self) -> Vec<MenuStructure> {
        vec![
            MenuStructure::Category {
                name: "USD".to_string(),
                items: vec![
                    MenuStructure::Category {
                        name: "Stage".to_string(),
                        items: vec![
                            MenuStructure::Node {
                                name: "Create Stage".to_string(),
                                node_type: "USD_CreateStage".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_CreateStage",
                                    "Create Stage",
                                    NodeCategory::new(&["USD", "Stage"]),
                                    "Create a new USD stage"
                                ),
                            },
                            MenuStructure::Node {
                                name: "Load Stage".to_string(),
                                node_type: "USD_LoadStage".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_LoadStage",
                                    "Load Stage",
                                    NodeCategory::new(&["USD", "Stage"]),
                                    "Load a USD stage from file"
                                ),
                            },
                            MenuStructure::Node {
                                name: "Save Stage".to_string(),
                                node_type: "USD_SaveStage".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_SaveStage",
                                    "Save Stage",
                                    NodeCategory::new(&["USD", "Stage"]),
                                    "Save USD stage to file"
                                ),
                            },
                        ],
                    },
                    MenuStructure::Category {
                        name: "Geometry".to_string(),
                        items: vec![
                            MenuStructure::Node {
                                name: "Mesh".to_string(),
                                node_type: "USD_Mesh".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_Mesh",
                                    "Mesh",
                                    NodeCategory::new(&["USD", "Geometry"]),
                                    "Create USD mesh geometry"
                                ),
                            },
                            MenuStructure::Node {
                                name: "Sphere".to_string(),
                                node_type: "USD_Sphere".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_Sphere",
                                    "Sphere",
                                    NodeCategory::new(&["USD", "Geometry"]),
                                    "Create USD sphere primitive"
                                ),
                            },
                            MenuStructure::Node {
                                name: "Cube".to_string(),
                                node_type: "USD_Cube".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_Cube",
                                    "Cube",
                                    NodeCategory::new(&["USD", "Geometry"]),
                                    "Create USD cube primitive"
                                ),
                            },
                            MenuStructure::Node {
                                name: "Cylinder".to_string(),
                                node_type: "USD_Cylinder".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_Cylinder",
                                    "Cylinder",
                                    NodeCategory::new(&["USD", "Geometry"]),
                                    "Create USD cylinder primitive"
                                ),
                            },
                        ],
                    },
                    MenuStructure::Category {
                        name: "Transform".to_string(),
                        items: vec![
                            MenuStructure::Node {
                                name: "Xform".to_string(),
                                node_type: "USD_Xform".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_Xform",
                                    "Xform",
                                    NodeCategory::new(&["USD", "Transform"]),
                                    "Apply transformation to USD prim"
                                ),
                            },
                            MenuStructure::Node {
                                name: "Translate".to_string(),
                                node_type: "USD_Translate".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_Translate",
                                    "Translate",
                                    NodeCategory::new(&["USD", "Transform"]),
                                    "Translate USD prim"
                                ),
                            },
                            MenuStructure::Node {
                                name: "Rotate".to_string(),
                                node_type: "USD_Rotate".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_Rotate",
                                    "Rotate",
                                    NodeCategory::new(&["USD", "Transform"]),
                                    "Rotate USD prim"
                                ),
                            },
                            MenuStructure::Node {
                                name: "Scale".to_string(),
                                node_type: "USD_Scale".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_Scale",
                                    "Scale",
                                    NodeCategory::new(&["USD", "Transform"]),
                                    "Scale USD prim"
                                ),
                            },
                        ],
                    },
                    MenuStructure::Category {
                        name: "Lighting".to_string(),
                        items: vec![
                            MenuStructure::Node {
                                name: "Distant Light".to_string(),
                                node_type: "USD_DistantLight".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_DistantLight",
                                    "Distant Light",
                                    NodeCategory::new(&["USD", "Lighting"]),
                                    "Create distant (directional) light"
                                ),
                            },
                            MenuStructure::Node {
                                name: "Sphere Light".to_string(),
                                node_type: "USD_SphereLight".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_SphereLight",
                                    "Sphere Light",
                                    NodeCategory::new(&["USD", "Lighting"]),
                                    "Create sphere area light"
                                ),
                            },
                            MenuStructure::Node {
                                name: "Dome Light".to_string(),
                                node_type: "USD_DomeLight".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_DomeLight",
                                    "Dome Light",
                                    NodeCategory::new(&["USD", "Lighting"]),
                                    "Create dome/environment light"
                                ),
                            },
                        ],
                    },
                    MenuStructure::Category {
                        name: "Shading".to_string(),
                        items: vec![
                            MenuStructure::Node {
                                name: "Material".to_string(),
                                node_type: "USD_Material".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_Material",
                                    "Material",
                                    NodeCategory::new(&["USD", "Shading"]),
                                    "Create USD material"
                                ),
                            },
                            MenuStructure::Node {
                                name: "Shader".to_string(),
                                node_type: "USD_Shader".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_Shader",
                                    "Shader",
                                    NodeCategory::new(&["USD", "Shading"]),
                                    "Create USD shader"
                                ),
                            },
                            MenuStructure::Node {
                                name: "Texture".to_string(),
                                node_type: "USD_Texture".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_Texture",
                                    "Texture",
                                    NodeCategory::new(&["USD", "Shading"]),
                                    "Create USD texture"
                                ),
                            },
                        ],
                    },
                    MenuStructure::Category {
                        name: "Viewport".to_string(),
                        items: vec![
                            MenuStructure::Node {
                                name: "USD Viewport".to_string(),
                                node_type: "USD_Viewport".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_Viewport",
                                    "USD Viewport",
                                    NodeCategory::new(&["USD", "Viewport"]),
                                    "3D viewport for visualizing and rendering USD stages with real-time navigation"
                                ).with_panel_type(PanelType::Viewport),
                            },
                            MenuStructure::Node {
                                name: "Stage Inspector".to_string(),
                                node_type: "USD_StageInspector".to_string(),
                                metadata: NodeMetadata::new(
                                    "USD_StageInspector",
                                    "Stage Inspector",
                                    NodeCategory::new(&["USD", "Viewport"]),
                                    "Inspect USD stage hierarchy"
                                ),
                            },
                        ],
                    },
                ],
            },
        ]
    }
    
    fn on_load(&self) -> Result<(), PluginError> {
        println!("USD Plugin loaded - comprehensive USD support available");
        Ok(())
    }
    
    fn on_unload(&self) -> Result<(), PluginError> {
        println!("USD Plugin unloaded");
        Ok(())
    }
}

// Simple node factory implementations for all USD node types

// Stage node factories
#[derive(Debug, Default)]
pub struct USDCreateStageFactory;

impl NodeFactory for USDCreateStageFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_CreateStage",
            "Create Stage",
            NodeCategory::new(&["USD", "Stage"]),
            "Create a new USD stage"
        )
        .with_color(Color32::from_rgb(80, 150, 200))
        .with_icon("🎬")
        .with_outputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("Created USD stage"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_CreateStage", "Create Stage", position))
    }
}

#[derive(Debug, Default)]
pub struct USDLoadStageFactory;

impl NodeFactory for USDLoadStageFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_LoadStage",
            "Load Stage",
            NodeCategory::new(&["USD", "Stage"]),
            "Load a USD stage from file"
        )
        .with_color(Color32::from_rgb(80, 150, 200))
        .with_icon("📂")
        .with_inputs(vec![
            PortDefinition::optional("File Path", DataType::String)
                .with_description("Path to USD file"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("Loaded USD stage"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_LoadStage", "Load Stage", position))
    }
}

#[derive(Debug, Default)]
pub struct USDSaveStageFactory;

impl NodeFactory for USDSaveStageFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_SaveStage",
            "Save Stage",
            NodeCategory::new(&["USD", "Stage"]),
            "Save USD stage to file"
        )
        .with_color(Color32::from_rgb(80, 150, 200))
        .with_icon("💾")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage to save"),
            PortDefinition::optional("File Path", DataType::String)
                .with_description("Output file path"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Success", DataType::Boolean)
                .with_description("Save operation success"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_SaveStage", "Save Stage", position))
    }
}

// Geometry node factories
#[derive(Debug, Default)]
pub struct USDMeshFactory;

impl NodeFactory for USDMeshFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_Mesh",
            "Mesh",
            NodeCategory::new(&["USD", "Geometry"]),
            "Create USD mesh geometry"
        )
        .with_color(Color32::from_rgb(100, 180, 100))
        .with_icon("🔺")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Mesh", DataType::String)
                .with_description("USD mesh prim"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_Mesh", "Mesh", position))
    }
}

#[derive(Debug, Default)]
pub struct USDSphereFactory;

impl NodeFactory for USDSphereFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_Sphere",
            "Sphere",
            NodeCategory::new(&["USD", "Geometry"]),
            "Create USD sphere primitive"
        )
        .with_color(Color32::from_rgb(100, 180, 100))
        .with_icon("🔴")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Sphere", DataType::String)
                .with_description("USD sphere prim"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_Sphere", "Sphere", position))
    }
}

#[derive(Debug, Default)]
pub struct USDCubeFactory;

impl NodeFactory for USDCubeFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_Cube",
            "Cube",
            NodeCategory::new(&["USD", "Geometry"]),
            "Create USD cube primitive"
        )
        .with_color(Color32::from_rgb(100, 180, 100))
        .with_icon("🔳")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Cube", DataType::String)
                .with_description("USD cube prim"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_Cube", "Cube", position))
    }
}

#[derive(Debug, Default)]
pub struct USDCylinderFactory;

impl NodeFactory for USDCylinderFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_Cylinder",
            "Cylinder",
            NodeCategory::new(&["USD", "Geometry"]),
            "Create USD cylinder primitive"
        )
        .with_color(Color32::from_rgb(100, 180, 100))
        .with_icon("🛢")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Cylinder", DataType::String)
                .with_description("USD cylinder prim"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_Cylinder", "Cylinder", position))
    }
}

// Transform node factories
#[derive(Debug, Default)]
pub struct USDXformFactory;

impl NodeFactory for USDXformFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_Xform",
            "Xform",
            NodeCategory::new(&["USD", "Transform"]),
            "Apply transformation to USD prim"
        )
        .with_color(Color32::from_rgb(150, 120, 200))
        .with_icon("🔄")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Xform", DataType::String)
                .with_description("USD Xform prim"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_Xform", "Xform", position))
    }
}

#[derive(Debug, Default)]
pub struct USDTranslateFactory;

impl NodeFactory for USDTranslateFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_Translate",
            "Translate",
            NodeCategory::new(&["USD", "Transform"]),
            "Translate USD prim"
        )
        .with_color(Color32::from_rgb(150, 120, 200))
        .with_icon("📍")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Translate", DataType::String)
                .with_description("USD translate transform"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_Translate", "Translate", position))
    }
}

#[derive(Debug, Default)]
pub struct USDRotateFactory;

impl NodeFactory for USDRotateFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_Rotate",
            "Rotate",
            NodeCategory::new(&["USD", "Transform"]),
            "Rotate USD prim"
        )
        .with_color(Color32::from_rgb(150, 120, 200))
        .with_icon("🔁")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Rotate", DataType::String)
                .with_description("USD rotate transform"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_Rotate", "Rotate", position))
    }
}

#[derive(Debug, Default)]
pub struct USDScaleFactory;

impl NodeFactory for USDScaleFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_Scale",
            "Scale",
            NodeCategory::new(&["USD", "Transform"]),
            "Scale USD prim"
        )
        .with_color(Color32::from_rgb(150, 120, 200))
        .with_icon("📏")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Scale", DataType::String)
                .with_description("USD scale transform"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_Scale", "Scale", position))
    }
}

// Lighting node factories
#[derive(Debug, Default)]
pub struct USDDistantLightFactory;

impl NodeFactory for USDDistantLightFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_DistantLight",
            "Distant Light",
            NodeCategory::new(&["USD", "Lighting"]),
            "Create distant (directional) light"
        )
        .with_color(Color32::from_rgb(200, 200, 100))
        .with_icon("☀️")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Light", DataType::String)
                .with_description("USD distant light"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_DistantLight", "Distant Light", position))
    }
}

#[derive(Debug, Default)]
pub struct USDSphereLightFactory;

impl NodeFactory for USDSphereLightFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_SphereLight",
            "Sphere Light",
            NodeCategory::new(&["USD", "Lighting"]),
            "Create sphere area light"
        )
        .with_color(Color32::from_rgb(200, 200, 100))
        .with_icon("💡")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Light", DataType::String)
                .with_description("USD sphere light"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_SphereLight", "Sphere Light", position))
    }
}

#[derive(Debug, Default)]
pub struct USDDomeLightFactory;

impl NodeFactory for USDDomeLightFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_DomeLight",
            "Dome Light",
            NodeCategory::new(&["USD", "Lighting"]),
            "Create dome/environment light"
        )
        .with_color(Color32::from_rgb(200, 200, 100))
        .with_icon("🌐")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Light", DataType::String)
                .with_description("USD dome light"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_DomeLight", "Dome Light", position))
    }
}

// Shading node factories
#[derive(Debug, Default)]
pub struct USDMaterialFactory;

impl NodeFactory for USDMaterialFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_Material",
            "Material",
            NodeCategory::new(&["USD", "Shading"]),
            "Create USD material"
        )
        .with_color(Color32::from_rgb(180, 100, 180))
        .with_icon("🎨")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Material", DataType::String)
                .with_description("USD material"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_Material", "Material", position))
    }
}

#[derive(Debug, Default)]
pub struct USDShaderFactory;

impl NodeFactory for USDShaderFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_Shader",
            "Shader",
            NodeCategory::new(&["USD", "Shading"]),
            "Create USD shader"
        )
        .with_color(Color32::from_rgb(180, 100, 180))
        .with_icon("🔮")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Shader", DataType::String)
                .with_description("USD shader"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_Shader", "Shader", position))
    }
}

#[derive(Debug, Default)]
pub struct USDTextureFactory;

impl NodeFactory for USDTextureFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_Texture",
            "Texture",
            NodeCategory::new(&["USD", "Shading"]),
            "Create USD texture"
        )
        .with_color(Color32::from_rgb(180, 100, 180))
        .with_icon("🖼️")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Texture", DataType::String)
                .with_description("USD texture"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_Texture", "Texture", position))
    }
}

// Stage Inspector factory
#[derive(Debug, Default)]
pub struct USDStageInspectorFactory;

impl NodeFactory for USDStageInspectorFactory {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_StageInspector",
            "Stage Inspector",
            NodeCategory::new(&["USD", "Viewport"]),
            "Inspect USD stage hierarchy"
        )
        .with_color(Color32::from_rgb(120, 120, 120))
        .with_icon("🔍")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD stage to inspect"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Info", DataType::String)
                .with_description("Stage information"),
        ])
        .with_workspace_compatibility(vec!["3D"])
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(SimpleUSDNode::new("USD_StageInspector", "Stage Inspector", position))
    }
}

// Simple generic USD node implementation
#[derive(Debug)]
pub struct SimpleUSDNode {
    pub id: String,
    pub position: Pos2,
    pub node_type: String,
    pub display_name: String,
}

impl SimpleUSDNode {
    pub fn new(node_type: &str, display_name: &str, position: Pos2) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            position,
            node_type: node_type.to_string(),
            display_name: display_name.to_string(),
        }
    }
}

impl PluginNode for SimpleUSDNode {
    fn id(&self) -> String { self.id.clone() }
    fn position(&self) -> Pos2 { self.position }
    fn set_position(&mut self, position: Pos2) { self.position = position; }
    
    fn render_parameters(&mut self, ui: &mut Ui) -> Vec<ParameterChange> {
        ui.label(format!("🎭 {}", self.display_name));
        ui.separator();
        ui.label(format!("Node Type: {}", self.node_type));
        ui.label("Parameters will be implemented soon...");
        Vec::new()
    }
    
    fn get_parameter(&self, _name: &str) -> Option<NodeData> {
        None
    }
    
    fn set_parameter(&mut self, _name: &str, _value: NodeData) {
        // TODO: Implement parameter setting
    }
    
    fn process(&mut self, _inputs: &HashMap<String, NodeData>) -> HashMap<String, NodeData> {
        HashMap::new()
    }
}

// Export C functions
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn NodePlugin {
    Box::into_raw(Box::new(USDPlugin))
}

#[no_mangle]
pub extern "C" fn destroy_plugin(plugin: *mut dyn NodePlugin) {
    unsafe { let _ = Box::from_raw(plugin); }
}