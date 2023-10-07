use std::ffi::CString;
use std::ptr::null;
use vulkan_sys::{
    vkCreateInstance, VkApplicationInfo, VkInstanceCreateInfo, VK_API_VERSION_1_0, VK_MAKE_VERSION,
    VK_STRUCTURE_TYPE_APPLICATION_INFO, VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO, VK_SUCCESS,
};

use crate::{prelude::EngineError, Engine};

impl Engine {
    fn create_instance(&mut self) -> Result<(), EngineError> {
        let application_name = CString::new("Hello, World!").unwrap();
        let engine_name = CString::new("Starstruck").unwrap();
        let app_info = VkApplicationInfo {
            sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: null(),
            pApplicationName: application_name.as_ptr(),
            applicationVersion: VK_MAKE_VERSION(1, 0, 0),
            pEngineName: engine_name.as_ptr(),
            engineVersion: VK_MAKE_VERSION(1, 0, 0),
            apiVersion: VK_API_VERSION_1_0,
        };

        let create_info = VkInstanceCreateInfo {
            sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            pApplicationInfo: &app_info,
            enabledLayerCount: 0,
            ppEnabledLayerNames: null(),
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: null(),
        };

        let result = unsafe { vkCreateInstance(&create_info, null(), &mut self.instance) };
        if result != VK_SUCCESS {
            return Err(EngineError::VulkanError(result));
        }

        Ok(())
    }
}
