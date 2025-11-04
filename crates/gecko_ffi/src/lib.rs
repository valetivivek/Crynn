use std::ffi::CString;
use std::os::raw::c_void;
use anyhow::Result;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use bindings::*;

pub struct GeckoEngine {
    window: *mut c_void,
    initialized: bool,
}

pub struct GeckoTab {
    tab: *mut c_void,
    parent_window: *mut c_void,
}

impl GeckoEngine {
    pub fn new() -> Result<Self> {
        unsafe {
            if gecko_init() != 0 {
                return Err(anyhow::anyhow!("Failed to initialize Gecko"));
            }
        }
        
        Ok(Self {
            window: std::ptr::null_mut(),
            initialized: true,
        })
    }

    pub fn create_window(&mut self, width: i32, height: i32, title: &str) -> Result<()> {
        let title_cstr = CString::new(title)?;
        unsafe {
            self.window = gecko_create_window(width, height, title_cstr.as_ptr());
            if self.window.is_null() {
                return Err(anyhow::anyhow!("Failed to create Gecko window"));
            }
        }
        Ok(())
    }

    pub fn navigate_to(&self, url: &str) -> Result<()> {
        if self.window.is_null() {
            return Err(anyhow::anyhow!("Window not created"));
        }
        
        let url_cstr = CString::new(url)?;
        unsafe {
            if gecko_navigate_to(self.window, url_cstr.as_ptr()) != 0 {
                return Err(anyhow::anyhow!("Navigation failed"));
            }
        }
        Ok(())
    }

    pub fn go_back(&self) -> Result<()> {
        if self.window.is_null() {
            return Err(anyhow::anyhow!("Window not created"));
        }
        
        unsafe {
            if gecko_go_back(self.window) != 0 {
                return Err(anyhow::anyhow!("Go back failed"));
            }
        }
        Ok(())
    }

    pub fn go_forward(&self) -> Result<()> {
        if self.window.is_null() {
            return Err(anyhow::anyhow!("Window not created"));
        }
        
        unsafe {
            if gecko_go_forward(self.window) != 0 {
                return Err(anyhow::anyhow!("Go forward failed"));
            }
        }
        Ok(())
    }

    pub fn reload(&self) -> Result<()> {
        if self.window.is_null() {
            return Err(anyhow::anyhow!("Window not created"));
        }
        
        unsafe {
            if gecko_reload(self.window) != 0 {
                return Err(anyhow::anyhow!("Reload failed"));
            }
        }
        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        if self.window.is_null() {
            return Err(anyhow::anyhow!("Window not created"));
        }
        
        unsafe {
            if gecko_stop(self.window) != 0 {
                return Err(anyhow::anyhow!("Stop failed"));
            }
        }
        Ok(())
    }

    pub fn get_memory_usage(&self) -> Result<usize> {
        if self.window.is_null() {
            return Err(anyhow::anyhow!("Window not created"));
        }
        
        unsafe {
            Ok(gecko_get_memory_usage(self.window))
        }
    }

    pub fn garbage_collect(&self) -> Result<()> {
        if self.window.is_null() {
            return Err(anyhow::anyhow!("Window not created"));
        }
        
        unsafe {
            gecko_garbage_collect(self.window);
        }
        Ok(())
    }
}

impl Drop for GeckoEngine {
    fn drop(&mut self) {
        if !self.window.is_null() {
            unsafe {
                gecko_destroy_window(self.window);
            }
        }
        if self.initialized {
            unsafe {
                gecko_shutdown();
            }
        }
    }
}

impl GeckoTab {
    pub fn new(parent_window: *mut c_void) -> Result<Self> {
        unsafe {
            let tab = gecko_create_tab(parent_window);
            if tab.is_null() {
                return Err(anyhow::anyhow!("Failed to create tab"));
            }
            Ok(Self { tab, parent_window })
        }
    }
}

impl Drop for GeckoTab {
    fn drop(&mut self) {
        unsafe {
            gecko_close_tab(self.parent_window, self.tab);
        }
    }
}
