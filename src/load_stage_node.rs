//! Proper USD Load Stage node implementation

use nodle_plugin_sdk::*;
use std::collections::HashMap;

/// USD Load Stage node with file loading functionality
pub struct USDLoadStageNode {
    id: String,
    position: Pos2,
    file_path: String,
    auto_reload: bool,
    load_payloads: bool,
}

impl USDLoadStageNode {
    pub fn new(position: Pos2) -> Self {
        Self {
            id: format!("usd_load_stage_{}", uuid()),
            position,
            file_path: String::new(),
            auto_reload: false,
            load_payloads: true,
        }
    }
}

impl PluginNode for USDLoadStageNode {
    fn id(&self) -> String {
        self.id.clone()
    }
    
    fn position(&self) -> Pos2 {
        self.position
    }
    
    fn set_position(&mut self, position: Pos2) {
        self.position = position;
    }
    
    fn get_parameter_ui(&self) -> ParameterUI {
        println!("🔥 USD Plugin: get_parameter_ui called!");
        println!("🔥 USD Plugin: self pointer: {:p}", self);
        println!("🔥 USD Plugin: self.id = {}", self.id);
        
        let mut elements = Vec::new();
        
        // Add some basic UI elements
        elements.push(UIElement::Heading("USD Load Stage".to_string()));
        elements.push(UIElement::Separator);
        
        elements.push(UIElement::TextEdit {
            label: "File Path".to_string(),
            value: self.file_path.clone(),
            parameter_name: "file_path".to_string(),
        });
        
        elements.push(UIElement::Button {
            label: "Browse...".to_string(),
            action: "browse_file".to_string(),
        });
        
        elements.push(UIElement::Checkbox {
            label: "Auto Reload".to_string(),
            value: self.auto_reload,
            parameter_name: "auto_reload".to_string(),
        });
        
        elements.push(UIElement::Checkbox {
            label: "Load Payloads".to_string(),
            value: self.load_payloads,
            parameter_name: "load_payloads".to_string(),
        });
        
        let result = ParameterUI { elements };
        
        println!("🔥 USD Plugin: get_parameter_ui returning with {} elements!", result.elements.len());
        result
    }
    
    fn handle_ui_action(&mut self, action: UIAction) -> Vec<ParameterChange> {
        let mut changes = Vec::new();
        
        match action {
            UIAction::ParameterChanged { parameter, value } => {
                match parameter.as_str() {
                    "file_path" => {
                        if let Some(path) = value.as_string() {
                            self.file_path = path.to_string();
                            changes.push(ParameterChange {
                                parameter: "file_path".to_string(),
                                value: NodeData::String(self.file_path.clone()),
                            });
                        }
                    }
                    "auto_reload" => {
                        if let Some(val) = value.as_boolean() {
                            self.auto_reload = val;
                            changes.push(ParameterChange {
                                parameter: "auto_reload".to_string(),
                                value: NodeData::Boolean(self.auto_reload),
                            });
                        }
                    }
                    "load_payloads" => {
                        if let Some(val) = value.as_boolean() {
                            self.load_payloads = val;
                            changes.push(ParameterChange {
                                parameter: "load_payloads".to_string(),
                                value: NodeData::Boolean(self.load_payloads),
                            });
                        }
                    }
                    _ => {}
                }
            }
            UIAction::ButtonClicked { action } => {
                match action.as_str() {
                    "browse_file" => {
                        // TODO: Open file dialog
                        // For now, use the test scene
                        self.file_path = "/Users/brian/nodle-claude/nodle-plugin-cycles/test_scene.usd".to_string();
                        changes.push(ParameterChange {
                            parameter: "file_path".to_string(),
                            value: NodeData::String(self.file_path.clone()),
                        });
                    }
                    _ => {}
                }
            }
        }
        
        changes
    }
    
    fn get_parameter(&self, name: &str) -> Option<NodeData> {
        match name {
            "file_path" => Some(NodeData::String(self.file_path.clone())),
            "auto_reload" => Some(NodeData::Boolean(self.auto_reload)),
            "load_payloads" => Some(NodeData::Boolean(self.load_payloads)),
            _ => None,
        }
    }
    
    fn set_parameter(&mut self, name: &str, value: NodeData) {
        match name {
            "file_path" => {
                if let Some(path) = value.as_string() {
                    self.file_path = path.to_string();
                }
            }
            "auto_reload" => {
                if let Some(reload) = value.as_boolean() {
                    self.auto_reload = reload;
                }
            }
            "load_payloads" => {
                if let Some(payloads) = value.as_boolean() {
                    self.load_payloads = payloads;
                }
            }
            _ => {}
        }
    }
    
    fn process(&mut self, _inputs: &HashMap<String, NodeData>) -> HashMap<String, NodeData> {
        let mut outputs = HashMap::new();
        
        if !self.file_path.is_empty() && std::path::Path::new(&self.file_path).exists() {
            // Output the USD file path for downstream nodes
            outputs.insert("Stage".to_string(), NodeData::String(self.file_path.clone()));
        }
        
        outputs
    }
}

/// Simple UUID generation
fn uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", timestamp)
}