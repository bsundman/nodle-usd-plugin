//! USD Viewport node - provides USD scene data for core rendering
//! 
//! This module only handles USD-specific logic and provides viewport data
//! to the core. The core handles all egui and wgpu rendering.

use nodle_plugin_sdk::*;
use std::collections::HashMap;

/// USD Viewport node - provides USD scene data for 3D visualization
#[derive(Debug, Clone)]
pub struct USDViewport {
    pub current_stage: String,
    pub viewport_data: ViewportData,
    pub camera_settings: CameraSettings,
}

/// USD-specific camera settings
#[derive(Debug, Clone)]
pub struct CameraSettings {
    pub orbit_sensitivity: f32,
    pub pan_sensitivity: f32,
    pub zoom_sensitivity: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            orbit_sensitivity: 0.5,
            pan_sensitivity: 1.0,
            zoom_sensitivity: 1.0,
        }
    }
}

impl Default for USDViewport {
    fn default() -> Self {
        Self {
            current_stage: String::new(),
            viewport_data: ViewportData::default(),
            camera_settings: CameraSettings::default(),
        }
    }
}

impl USDViewport {
    /// Load USD stage and convert to scene data
    pub fn load_stage(&mut self, stage_path: &str) {
        println!("USD Plugin: Loading stage: {}", stage_path);
        
        // TODO: Implement actual USD stage loading
        // For now, create a simple test scene
        let mut scene = SceneData::default();
        scene.name = format!("USD Stage: {}", stage_path);
        
        // Create a simple cube mesh as placeholder
        let cube_mesh = MeshData {
            id: "cube".to_string(),
            vertices: vec![
                // Front face
                -1.0, -1.0,  1.0,
                 1.0, -1.0,  1.0,
                 1.0,  1.0,  1.0,
                -1.0,  1.0,  1.0,
                // Back face
                -1.0, -1.0, -1.0,
                -1.0,  1.0, -1.0,
                 1.0,  1.0, -1.0,
                 1.0, -1.0, -1.0,
            ],
            normals: vec![
                // Front face
                0.0, 0.0, 1.0,
                0.0, 0.0, 1.0,
                0.0, 0.0, 1.0,
                0.0, 0.0, 1.0,
                // Back face
                0.0, 0.0, -1.0,
                0.0, 0.0, -1.0,
                0.0, 0.0, -1.0,
                0.0, 0.0, -1.0,
            ],
            uvs: vec![
                0.0, 0.0,
                1.0, 0.0,
                1.0, 1.0,
                0.0, 1.0,
                0.0, 0.0,
                1.0, 0.0,
                1.0, 1.0,
                0.0, 1.0,
            ],
            indices: vec![
                // Front face
                0, 1, 2,   2, 3, 0,
                // Back face
                4, 5, 6,   6, 7, 4,
                // Top face
                3, 2, 6,   6, 5, 3,
                // Bottom face
                0, 4, 7,   7, 1, 0,
                // Right face
                1, 7, 6,   6, 2, 1,
                // Left face
                4, 0, 3,   3, 5, 4,
            ],
            material_id: Some("usd_material".to_string()),
            transform: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        
        scene.meshes.push(cube_mesh);
        
        // Create a simple material
        let material = MaterialData {
            id: "usd_material".to_string(),
            name: "USD Material".to_string(),
            base_color: [0.7, 0.7, 0.9, 1.0],
            metallic: 0.0,
            roughness: 0.5,
            emission: [0.0, 0.0, 0.0],
            diffuse_texture: None,
            normal_texture: None,
            roughness_texture: None,
            metallic_texture: None,
        };
        
        scene.materials.push(material);
        
        // Add a simple directional light
        let light = LightData {
            id: "sun".to_string(),
            light_type: LightType::Directional,
            position: [0.0, 10.0, 5.0],
            direction: [-0.5, -1.0, -0.5],
            color: [1.0, 1.0, 0.9],
            intensity: 5.0,
            range: 100.0,
            spot_angle: 0.0,
        };
        
        scene.lights.push(light);
        
        // Set scene bounding box
        scene.bounding_box = Some(([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]));
        
        self.viewport_data.scene = scene;
        self.viewport_data.scene_dirty = true;
        self.current_stage = stage_path.to_string();
    }
    
    /// Handle camera manipulation with USD-specific behavior
    pub fn handle_camera_manipulation(&mut self, manipulation: CameraManipulation) {
        let camera = &mut self.viewport_data.scene.camera;
        
        match manipulation {
            CameraManipulation::Orbit { delta_x, delta_y } => {
                let radius = ((camera.position[0] - camera.target[0]).powi(2) + 
                             (camera.position[1] - camera.target[1]).powi(2) + 
                             (camera.position[2] - camera.target[2]).powi(2)).sqrt();
                
                // Convert to spherical coordinates
                let mut theta = (camera.position[2] - camera.target[2]).atan2(camera.position[0] - camera.target[0]);
                let mut phi = ((camera.position[1] - camera.target[1]) / radius).asin();
                
                // Apply orbit deltas
                theta += delta_x * self.camera_settings.orbit_sensitivity;
                phi += delta_y * self.camera_settings.orbit_sensitivity;
                
                // Clamp phi to prevent gimbal lock
                phi = phi.clamp(-std::f32::consts::PI * 0.49, std::f32::consts::PI * 0.49);
                
                // Convert back to Cartesian
                camera.position[0] = camera.target[0] + radius * phi.cos() * theta.cos();
                camera.position[1] = camera.target[1] + radius * phi.sin();
                camera.position[2] = camera.target[2] + radius * phi.cos() * theta.sin();
            }
            CameraManipulation::Pan { delta_x, delta_y } => {
                // Calculate camera right and up vectors
                let forward = [
                    camera.target[0] - camera.position[0],
                    camera.target[1] - camera.position[1],
                    camera.target[2] - camera.position[2]
                ];
                let right = [
                    forward[1] * camera.up[2] - forward[2] * camera.up[1],
                    forward[2] * camera.up[0] - forward[0] * camera.up[2],
                    forward[0] * camera.up[1] - forward[1] * camera.up[0]
                ];
                
                // Pan both position and target
                let pan_x = delta_x * self.camera_settings.pan_sensitivity;
                let pan_y = delta_y * self.camera_settings.pan_sensitivity;
                
                for i in 0..3 {
                    camera.position[i] += right[i] * pan_x + camera.up[i] * pan_y;
                    camera.target[i] += right[i] * pan_x + camera.up[i] * pan_y;
                }
            }
            CameraManipulation::Zoom { delta } => {
                let direction = [
                    camera.target[0] - camera.position[0],
                    camera.target[1] - camera.position[1],
                    camera.target[2] - camera.position[2]
                ];
                
                let zoom_factor = delta * self.camera_settings.zoom_sensitivity;
                
                for i in 0..3 {
                    camera.position[i] += direction[i] * zoom_factor;
                }
            }
            CameraManipulation::Reset => {
                *camera = CameraData::default();
            }
            CameraManipulation::SetPosition { position, target } => {
                camera.position = position;
                camera.target = target;
            }
        }
        
        self.viewport_data.scene_dirty = true;
    }
}

impl NodeFactory for USDViewport {
    fn metadata(&self) -> NodeMetadata {
        NodeMetadata::new(
            "USD_Viewport",
            "USD Viewport", 
            NodeCategory::new(&["USD", "Viewport"]),
            "3D viewport for visualizing USD stages with Maya-style navigation"
        )
        .with_color(Color32::from_rgb(100, 200, 100))
        .with_icon("🎥")
        .with_inputs(vec![
            PortDefinition::required("Stage", DataType::String)
                .with_description("USD Stage to visualize"),
            PortDefinition::optional("Camera", DataType::String)
                .with_description("Camera prim for viewport (optional)"),
        ])
        .with_outputs(vec![
            PortDefinition::required("Rendered Image", DataType::String)
                .with_description("Viewport render output"),
        ])
        .with_workspace_compatibility(vec!["3D"])
        .with_panel_type(PanelType::Viewport)
    }
    
    fn create_node(&self, position: Pos2) -> Box<dyn PluginNode> {
        Box::new(USDViewportNode {
            id: uuid::Uuid::new_v4().to_string(),
            position,
            viewport_data: USDViewport::default(),
        })
    }
}

/// Plugin node wrapper for USD Viewport
pub struct USDViewportNode {
    pub id: String,
    pub position: Pos2,
    pub viewport_data: USDViewport,
}

impl std::fmt::Debug for USDViewportNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("USDViewportNode")
            .field("id", &self.id)
            .field("position", &self.position)
            .finish()
    }
}

impl PluginNode for USDViewportNode {
    fn id(&self) -> String {
        self.id.clone()
    }
    
    fn position(&self) -> Pos2 {
        self.position
    }
    
    fn set_position(&mut self, position: Pos2) {
        self.position = position;
    }
    
    fn render_parameters(&mut self, ui: &mut Ui) -> Vec<ParameterChange> {
        let mut changes = Vec::new();
        
        // USD Viewport Parameters - no direct egui rendering of 3D content
        ui.heading("USD Viewport Settings");
        ui.separator();
        
        // Stage Information
        ui.label("📁 Stage Information");
        if self.viewport_data.current_stage.is_empty() {
            ui.label("No USD stage loaded");
        } else {
            ui.label(format!("Current Stage: {}", self.viewport_data.current_stage));
        }
        ui.separator();
        
        // Camera Settings
        ui.label("🎥 Camera Settings");
        ui.horizontal(|ui| {
            ui.label("Orbit Sensitivity:");
            if ui.add(egui::Slider::new(&mut self.viewport_data.camera_settings.orbit_sensitivity, 0.1..=2.0)).changed() {
                changes.push(ParameterChange {
                    parameter: "orbit_sensitivity".to_string(),
                    value: NodeData::Float(self.viewport_data.camera_settings.orbit_sensitivity),
                });
            }
        });
        ui.horizontal(|ui| {
            ui.label("Pan Sensitivity:");
            if ui.add(egui::Slider::new(&mut self.viewport_data.camera_settings.pan_sensitivity, 0.1..=2.0)).changed() {
                changes.push(ParameterChange {
                    parameter: "pan_sensitivity".to_string(),
                    value: NodeData::Float(self.viewport_data.camera_settings.pan_sensitivity),
                });
            }
        });
        ui.horizontal(|ui| {
            ui.label("Zoom Sensitivity:");
            if ui.add(egui::Slider::new(&mut self.viewport_data.camera_settings.zoom_sensitivity, 0.1..=2.0)).changed() {
                changes.push(ParameterChange {
                    parameter: "zoom_sensitivity".to_string(),
                    value: NodeData::Float(self.viewport_data.camera_settings.zoom_sensitivity),
                });
            }
        });
        
        if ui.button("Reset Camera").clicked() {
            self.viewport_data.handle_camera_manipulation(CameraManipulation::Reset);
            changes.push(ParameterChange {
                parameter: "camera_reset".to_string(),
                value: NodeData::Boolean(true),
            });
        }
        ui.separator();
        
        // Viewport Settings
        ui.label("⚙️ Viewport Settings");
        if ui.checkbox(&mut self.viewport_data.viewport_data.settings.wireframe, "Wireframe").changed() {
            self.viewport_data.viewport_data.settings_dirty = true;
            changes.push(ParameterChange {
                parameter: "wireframe".to_string(),
                value: NodeData::Boolean(self.viewport_data.viewport_data.settings.wireframe),
            });
        }
        if ui.checkbox(&mut self.viewport_data.viewport_data.settings.lighting, "Lighting").changed() {
            self.viewport_data.viewport_data.settings_dirty = true;
            changes.push(ParameterChange {
                parameter: "lighting".to_string(),
                value: NodeData::Boolean(self.viewport_data.viewport_data.settings.lighting),
            });
        }
        if ui.checkbox(&mut self.viewport_data.viewport_data.settings.show_grid, "Show Grid").changed() {
            self.viewport_data.viewport_data.settings_dirty = true;
            changes.push(ParameterChange {
                parameter: "show_grid".to_string(),
                value: NodeData::Boolean(self.viewport_data.viewport_data.settings.show_grid),
            });
        }
        if ui.checkbox(&mut self.viewport_data.viewport_data.settings.show_ground_plane, "Show Ground Plane").changed() {
            self.viewport_data.viewport_data.settings_dirty = true;
            changes.push(ParameterChange {
                parameter: "show_ground_plane".to_string(),
                value: NodeData::Boolean(self.viewport_data.viewport_data.settings.show_ground_plane),
            });
        }
        
        ui.separator();
        ui.label("💡 USD Plugin - Data-driven viewport rendering");
        
        changes
    }
    
    fn get_parameter(&self, name: &str) -> Option<NodeData> {
        match name {
            "current_stage" => Some(NodeData::String(self.viewport_data.current_stage.clone())),
            "orbit_sensitivity" => Some(NodeData::Float(self.viewport_data.camera_settings.orbit_sensitivity)),
            "pan_sensitivity" => Some(NodeData::Float(self.viewport_data.camera_settings.pan_sensitivity)),
            "zoom_sensitivity" => Some(NodeData::Float(self.viewport_data.camera_settings.zoom_sensitivity)),
            "wireframe" => Some(NodeData::Boolean(self.viewport_data.viewport_data.settings.wireframe)),
            "lighting" => Some(NodeData::Boolean(self.viewport_data.viewport_data.settings.lighting)),
            "show_grid" => Some(NodeData::Boolean(self.viewport_data.viewport_data.settings.show_grid)),
            "show_ground_plane" => Some(NodeData::Boolean(self.viewport_data.viewport_data.settings.show_ground_plane)),
            _ => None,
        }
    }
    
    fn set_parameter(&mut self, name: &str, value: NodeData) {
        match name {
            "current_stage" => {
                if let Some(stage) = value.as_string() {
                    self.viewport_data.load_stage(stage);
                }
            }
            "orbit_sensitivity" => {
                if let Some(sensitivity) = value.as_float() {
                    self.viewport_data.camera_settings.orbit_sensitivity = sensitivity;
                }
            }
            "pan_sensitivity" => {
                if let Some(sensitivity) = value.as_float() {
                    self.viewport_data.camera_settings.pan_sensitivity = sensitivity;
                }
            }
            "zoom_sensitivity" => {
                if let Some(sensitivity) = value.as_float() {
                    self.viewport_data.camera_settings.zoom_sensitivity = sensitivity;
                }
            }
            "wireframe" => {
                if let Some(enabled) = value.as_boolean() {
                    self.viewport_data.viewport_data.settings.wireframe = enabled;
                    self.viewport_data.viewport_data.settings_dirty = true;
                }
            }
            "lighting" => {
                if let Some(enabled) = value.as_boolean() {
                    self.viewport_data.viewport_data.settings.lighting = enabled;
                    self.viewport_data.viewport_data.settings_dirty = true;
                }
            }
            "show_grid" => {
                if let Some(enabled) = value.as_boolean() {
                    self.viewport_data.viewport_data.settings.show_grid = enabled;
                    self.viewport_data.viewport_data.settings_dirty = true;
                }
            }
            "show_ground_plane" => {
                if let Some(enabled) = value.as_boolean() {
                    self.viewport_data.viewport_data.settings.show_ground_plane = enabled;
                    self.viewport_data.viewport_data.settings_dirty = true;
                }
            }
            _ => {}
        }
    }
    
    fn process(&mut self, inputs: &HashMap<String, NodeData>) -> HashMap<String, NodeData> {
        let mut outputs = HashMap::new();
        
        // Process USD stage input
        if let Some(stage_data) = inputs.get("Stage") {
            if let Some(stage_path) = stage_data.as_string() {
                if stage_path != self.viewport_data.current_stage {
                    self.viewport_data.load_stage(stage_path);
                    outputs.insert("Rendered Image".to_string(), 
                        NodeData::String(format!("USD Stage Loaded: {}", stage_path)));
                }
            }
        } else {
            // No stage connected - clear current stage
            if !self.viewport_data.current_stage.is_empty() {
                self.viewport_data.current_stage.clear();
                self.viewport_data.viewport_data.scene = SceneData::default();
                self.viewport_data.viewport_data.scene_dirty = true;
            }
        }
        
        // Handle camera input if provided
        if let Some(camera_data) = inputs.get("Camera") {
            if let Some(camera_path) = camera_data.as_string() {
                println!("USD Plugin: Using camera: {}", camera_path);
                // TODO: Extract camera from USD stage and apply to viewport
            }
        }
        
        outputs
    }
    
    /// Provide viewport data to the core for rendering
    fn get_viewport_data(&self) -> Option<ViewportData> {
        Some(self.viewport_data.viewport_data.clone())
    }
    
    /// Handle viewport camera manipulation
    fn handle_viewport_camera(&mut self, manipulation: CameraManipulation) {
        self.viewport_data.handle_camera_manipulation(manipulation);
    }
    
    /// Handle viewport settings changes
    fn handle_viewport_settings(&mut self, settings: ViewportSettings) {
        self.viewport_data.viewport_data.settings = settings;
        self.viewport_data.viewport_data.settings_dirty = true;
    }
    
    fn supports_viewport(&self) -> bool {
        true
    }
}