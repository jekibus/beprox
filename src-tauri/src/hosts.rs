use std::fs::{read_to_string, File};
use std::io::Write;

const START_MARKER: &str = "## BeProx - Start ##";
const END_MARKER: &str = "## BeProx - End ##";

#[cfg(target_os = "windows")]
const HOSTS_PATH: &str = r"C:\Windows\System32\drivers\etc\hosts";

#[cfg(not(target_os = "windows"))]
const HOSTS_PATH: &str = "/etc/hosts";

#[tauri::command]
pub fn add_host_entry(domain: &str) -> Result<(), String> {
    let content = read_to_string(HOSTS_PATH).map_err(|e| e.to_string())?;

    let entry = format!("127.0.0.1 {}", domain);

    // 1. If the block doesn't exist, create it at the end
    if !content.contains(START_MARKER) {
        let mut new_content = content.clone();
        if !new_content.ends_with('\n') {
            new_content.push('\n');
        }
        new_content.push_str(&format!("{}\n{}\n{}\n", START_MARKER, entry, END_MARKER));
        return write_hosts_file(&new_content);
    }

    // 2. If the block exists, parse it and ensure our domain is in it
    let parts: Vec<&str> = content.split(START_MARKER).collect();
    if parts.len() != 2 {
        return Err("Hosts file structure is confusing (multiple start markers). Manual intervention required.".into());
    }

    let pre_block = parts[0];
    let rest = parts[1];
    
    let end_parts: Vec<&str> = rest.split(END_MARKER).collect();
    if end_parts.len() != 2 {
        return Err("Hosts file structure is confusing (missing or multiple end markers).".into());
    }

    let block_content = end_parts[0];
    let post_block = end_parts[1];

    // Check if domain exists in the block
    if block_content.contains(&entry) {
        println!("Domain {} already exists in BeProx block", domain);
        return Ok(());
    }

    // Reconstruct the file with the new entry added to the block
    let new_block_content = format!("{}{}\n", block_content.trim_end(), if block_content.trim().is_empty() { "" } else { "\n" });
    // Note: We need to handle the case where block_content is just whitespace/newlines
    let final_block = format!("{}{}\n", new_block_content, entry);
    
    // Clean up multiple newlines if needed, but simple concatenation is usually safe for hosts
    let new_file_content = format!("{}{}{}{}{}", pre_block, START_MARKER, final_block, END_MARKER, post_block);

    write_hosts_file(&new_file_content)?;
    println!("Added {} to BeProx block in hosts file", domain);

    Ok(())
}

#[tauri::command]
pub fn remove_host_entry(domain: &str) -> Result<(), String> {
    let content = read_to_string(HOSTS_PATH).map_err(|e| e.to_string())?;
    let entry = format!("127.0.0.1 {}", domain);

    if !content.contains(START_MARKER) {
        return Ok(()); // Nothing to remove
    }

    let parts: Vec<&str> = content.split(START_MARKER).collect();
    if parts.len() != 2 {
         return Err("Hosts file structure is confusing (multiple start markers).".into());
    }
    let pre_block = parts[0];
    let rest = parts[1];
    let end_parts: Vec<&str> = rest.split(END_MARKER).collect();
    if end_parts.len() != 2 {
        return Err("Hosts file structure is confusing (missing end marker).".into());
    }
    let block_content = end_parts[0];
    let post_block = end_parts[1];

    if !block_content.contains(&entry) {
        return Ok(()); // Not found, nothing to do
    }

    // Filter out the line
    let new_block_content: String = block_content
        .lines()
        .filter(|line| !line.trim().contains(&entry))
        .collect::<Vec<&str>>()
        .join("\n");
    
    // Ensure we keep the block clean but valid
    // If new_block_content is just whitespace, we want just "\n"
    // If it has content, we want "content\n"
    
    let trimmed_content = new_block_content.trim();
    let final_block = if trimmed_content.is_empty() {
        "\n".to_string()
    } else {
        format!("\n{}\n", trimmed_content)
    };

    let new_file_content = format!("{}{}{}{}{}", pre_block, START_MARKER, final_block, END_MARKER, post_block);
    write_hosts_file(&new_file_content)?;
    println!("Removed {} from BeProx block in hosts file", domain);

    Ok(())
}

fn write_hosts_file(content: &str) -> Result<(), String> {
    let mut file = File::create(HOSTS_PATH)
        .map_err(|e| {
            #[cfg(target_os = "windows")]
            let msg = "Are you running as Administrator?";
            #[cfg(not(target_os = "windows"))]
            let msg = "Are you running with sudo?";
            
            format!("Failed to open hosts file for writing: {}. {}", e, msg)
        })?;
    file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}
