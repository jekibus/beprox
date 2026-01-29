use tauri::State;
use crate::models::{Site, Settings};
use crate::store::StoreState;
use crate::hosts;

#[tauri::command]
pub fn get_settings(state: State<StoreState>) -> Result<Settings, String> {
    state.settings.read()
        .map(|settings| settings.clone())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_settings(state: State<StoreState>, settings: Settings) -> Result<Settings, String> {
    state.update_settings(settings.clone())?;
    Ok(settings)
}

#[tauri::command]
pub fn get_sites(state: State<StoreState>) -> Result<Vec<Site>, String> {
    state.sites.read()
        .map(|sites| sites.clone())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_site(state: State<StoreState>, domain: String, port: u16) -> Result<Site, String> {
    // 1. Add to /etc/hosts
    hosts::add_host_entry(&domain)?;

    // 2. Add to store
    let site = Site {
        id: uuid::Uuid::new_v4().to_string(),
        domain,
        port,
        enabled: true,
    };
    
    state.add_site(site.clone())?;

    Ok(site)
}

#[tauri::command]
pub fn delete_site(state: State<StoreState>, id: String) -> Result<(), String> {
    // 1. Get site to find domain
    let site = {
        let sites = state.sites.read().map_err(|e| e.to_string())?;
        sites.iter().find(|s| s.id == id).cloned()
    };

    if let Some(site) = site {
        // 2. Remove from /etc/hosts
        hosts::remove_host_entry(&site.domain)?;

        // 3. Remove from store
        state.remove_site(&id)?;
    }

    Ok(())
}

#[tauri::command]
pub fn update_site(state: State<StoreState>, id: String, domain: String, port: u16) -> Result<Site, String> {
    // 1. Get old site
    let old_site = {
        let sites = state.sites.read().map_err(|e| e.to_string())?;
        sites.iter().find(|s| s.id == id).cloned()
    };

    if let Some(old_site) = old_site {
        // 2. If domain changed, update /etc/hosts
        if old_site.domain != domain {
            hosts::remove_host_entry(&old_site.domain)?;
            hosts::add_host_entry(&domain)?;
        }

        // 3. Update store
        let new_site = Site {
            id: id.clone(),
            domain,
            port,
            enabled: old_site.enabled,
        };
        state.update_site(&id, new_site.clone())?;
        
        Ok(new_site)
    } else {
        Err("Site not found".to_string())
    }
}

#[tauri::command]
pub fn toggle_site(state: State<StoreState>, id: String) -> Result<Site, String> {
    let mut site = {
        let sites = state.sites.read().map_err(|e| e.to_string())?;
        sites.iter().find(|s| s.id == id).cloned().ok_or("Site not found")?
    };

    site.enabled = !site.enabled;
    state.update_site(&id, site.clone())?;
    
    Ok(site)
}
