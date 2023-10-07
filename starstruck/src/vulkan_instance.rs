use std::ffi::CString;
use std::ptr::null;
use vk_sys::{
    createInstance, ApplicationInfo, InstanceCreateInfo,
    STRUCTURE_TYPE_APPLICATION_INFO, STRUCTURE_TYPE_INSTANCE_CREATE_INFO, SUCCESS
};

use crate::helpers::make_api_version;
use crate::{prelude::EngineError, Engine, plugins::{EnginePlugin, EngineCleanup}};

/// Makes a vulkan instance and manages the state around it
pub struct VulkanInstancePlugin;

impl EnginePlugin for VulkanInstancePlugin {
    fn plugin_make(&self, engine: &mut Engine) {
        self.create_instance(engine).unwrap();
    }
}

impl VulkanInstancePlugin {
    fn new() -> Self {
        Self
    }
    fn create_instance(&self, engine: &mut Engine) -> Result<(), EngineError> {
        let application_name = CString::new("Hello, World!").unwrap();
        let engine_name = CString::new("Starstruck").unwrap();
        let app_info = ApplicationInfo {
            sType: STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: null(),
            pApplicationName: application_name.as_ptr(),
            applicationVersion: make_api_version(0, 1, 0, 0),
            pEngineName: engine_name.as_ptr(),
            engineVersion: make_api_version(0, 1, 0, 0),
            apiVersion: make_api_version(0, 1, 0, 0),
        };

        let create_info = InstanceCreateInfo {
            sType: STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            pApplicationInfo: &app_info,
            enabledLayerCount: 0,
            ppEnabledLayerNames: null(),
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: null(),
        };

        let result = unsafe { createInstance(&create_info, null(), &mut engine.renderer.instance) };

        if result != SUCCESS {
            return Err(EngineError::VulkanError(result));
        }

        Ok(())
    }
}

impl EngineCleanup for VulkanInstancePlugin {}
