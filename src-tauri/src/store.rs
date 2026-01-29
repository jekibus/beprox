use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use crate::models::{Site, Settings};

const STORE_FILENAME: &str = "sites.json";
const SETTINGS_FILENAME: &str = "settings.json";

#[derive(Clone)]
pub struct StoreState {
    pub sites: Arc<RwLock<Vec<Site>>>,
    pub settings: Arc<RwLock<Settings>>,
    pub app_data_dir: PathBuf,
}

impl StoreState {
    pub fn new(app_data_dir: PathBuf) -> Self {
        let store_path = app_data_dir.join(STORE_FILENAME);
        let sites = if store_path.exists() {
            let content = fs::read_to_string(&store_path).unwrap_or_else(|_| "[]".to_string());
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };

        let settings_path = app_data_dir.join(SETTINGS_FILENAME);
        let settings = if settings_path.exists() {
            let content = fs::read_to_string(&settings_path).unwrap_or_else(|_| "{}".to_string());
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Settings::default()
        };

        Self {
            sites: Arc::new(RwLock::new(sites)),
            settings: Arc::new(RwLock::new(settings)),
            app_data_dir,
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let sites = self.sites.read().map_err(|e| e.to_string())?;
        let json = serde_json::to_string_pretty(&*sites).map_err(|e| e.to_string())?;
        
        if !self.app_data_dir.exists() {
            fs::create_dir_all(&self.app_data_dir).map_err(|e| e.to_string())?;
        }

        let store_path = self.app_data_dir.join(STORE_FILENAME);
        let mut file = File::create(store_path).map_err(|e| e.to_string())?;
        file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
        
        Ok(())
    }

    pub fn add_site(&self, site: Site) -> Result<(), String> {
        {
            let mut sites = self.sites.write().map_err(|e| e.to_string())?;
            sites.push(site);
        }
        self.save()
    }

    pub fn remove_site(&self, id: &str) -> Result<Option<Site>, String> {
        let removed_site = {
            let mut sites = self.sites.write().map_err(|e| e.to_string())?;
            if let Some(pos) = sites.iter().position(|s| s.id == id) {
                Some(sites.remove(pos))
            } else {
                None
            }
        };
        
        if removed_site.is_some() {
            self.save()?;
        }
        
        Ok(removed_site)
    }

    pub fn update_site(&self, id: &str, new_site: Site) -> Result<(), String> {
        {
            let mut sites = self.sites.write().map_err(|e| e.to_string())?;
            if let Some(site) = sites.iter_mut().find(|s| s.id == id) {
                *site = new_site;
            } else {
                return Err("Site not found".to_string());
            }
        }
        self.save()
    }
    
    pub fn get_site_by_domain(&self, domain: &str) -> Option<Site> {
        let sites = self.sites.read().ok()?;
        sites.iter().find(|s| s.domain == domain).cloned()
    }

    pub fn save_settings(&self) -> Result<(), String> {
        let settings = self.settings.read().map_err(|e| e.to_string())?;
        let json = serde_json::to_string_pretty(&*settings).map_err(|e| e.to_string())?;
        
        if !self.app_data_dir.exists() {
            fs::create_dir_all(&self.app_data_dir).map_err(|e| e.to_string())?;
        }

        let store_path = self.app_data_dir.join(SETTINGS_FILENAME);
        let mut file = File::create(store_path).map_err(|e| e.to_string())?;
        file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
        
        Ok(())
    }

    pub fn update_settings(&self, new_settings: Settings) -> Result<(), String> {
        {
            let mut settings = self.settings.write().map_err(|e| e.to_string())?;
            *settings = new_settings;
        }
        self.save_settings()
    }
}
