use std::io::{self, Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct Config {
    listen: String,
    index: String,
    pass: String,
    deny_files: Vec<String>,
    deny_extensions: Vec<String>,
    lb_algo: String,
    servers: Vec<String>,
    root: String,
    deny_directories: Vec<String>,
    allow_directories: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            listen: "3000".to_string(),
            index: "index.php".to_string(),
            pass: "".to_string(),
            deny_files: vec![],
            deny_extensions: vec![],
            lb_algo: "none".to_string(),
            servers: vec![],
            root: ".".to_string(),
            deny_directories: vec![],
            allow_directories: vec![],
        }
    }

    pub fn parse(&mut self, path: &str) -> io::Result<()> {
        let contents = std::fs::read_to_string(path)?;

        for line in contents.lines() {
            // Skip empty lines and comments
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Split the line into directive and value
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "listen" => {
                    if parts.len() != 2 {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid listen directive"));
                    }
                    self.listen = parts[1].to_string();
                }
                "index" => {
                    if parts.len() != 2 {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid index directive"));
                    }
                    self.index = parts[1].to_string();
                }
                "pass" => {
                    if parts.len() != 2 {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid pass directive"));
                    }
                    self.pass = parts[1].to_string();
                }
                "deny_files" => {
                    if parts.len() < 2 {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid deny_files directive"));
                    }
                    self.deny_files.extend(parts[1..].iter().map(|&s| s.to_string()));
                }
                "deny_extensions" => {
                    if parts.len() < 2 {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid deny_extensions directive"));
                    }
                    self.deny_extensions.extend(parts[1..].iter().map(|&s| s.to_string()));
                }
                "alb_algo" => {
                    if parts.len() != 2 {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid alb_algo directive"));
                    }
                    match parts[1] {
                        "none" | "roundrobin" | "leastconn" | "source" | "off" => {
                            self.lb_algo = parts[1].to_string();
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                "Invalid load balancing algorithm",
                            ));
                        }
                    }
                }
                "servers" => {
                    if parts.len() < 2 {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid servers directive"));
                    }
                    self.servers.extend(parts[1..].iter().map(|&s| s.to_string()));
                },
                "root" => {
                    if parts.len() != 2 {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid root directive"));
                    }
                    self.root = parts[1].to_string();
                },
                "deny_directories" => {
                    if parts.len() < 2 {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid deny_directories directive"));
                    }
                    self.deny_directories.extend(parts[1..].iter().map(|&s| s.to_string()));
                },
                "allow_directories" => {
                    if parts.len() < 2 {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid allow_directories directive"));
                    }
                    self.allow_directories.extend(parts[1..].iter().map(|&s| s.to_string()));
                },
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Unknown directive: {}", parts[0]),
                    ));
                }
            }
        }

        Ok(())
    }

    // Helper methods to access the configuration
    pub fn listen(&self) -> &str {
        &self.listen
    }

    pub fn index(&self) -> &str {
        &self.index
    }

    pub fn pass(&self) -> &str {
        &self.pass
    }

    pub fn deny_files(&self) -> &[String] {
        &self.deny_files
    }

    pub fn deny_extensions(&self) -> &[String] {
        &self.deny_extensions
    }

    pub fn lb_algo(&self) -> &str {
        &self.lb_algo
    }

    pub fn servers(&self) -> Vec<String> {
        self.servers.clone()
    }

    pub fn root(&self) -> &str {
        &self.root
    }

    pub fn deny_directories(&self) -> &[String] {
        &self.deny_directories
    }

    pub fn allow_directories(&self) -> &[String] {
        &self.allow_directories
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_parsing() -> io::Result<()> {
        let config_content = r#"
# Port to listen on
listen 3000
# Deny specific files and extensions
deny_files index.html other.html
deny_extensions html js css
# Index file to server
index index.php
pass /usr/bin/php
# Load balancing algorithm
alb_algo roundrobin
servers 127.0.0.1:3001 127.0.0.1:3002
# Root directory
root .
# Deny specific directories
deny_directories .git .svn
# Allow specific directories
allow_directories assets/images assets/css
"#;

        let temp_file = NamedTempFile::new()?;
        write(temp_file.path(), config_content)?;

        let mut config = Config::new();
        config.parse(temp_file.path().to_str().unwrap())?;

        assert_eq!(config.listen(), "3000");
        assert_eq!(config.index(), "index.php");
        assert_eq!(config.pass(), "/usr/bin/php");
        assert_eq!(config.deny_files(), &["index.html", "other.html"]);
        assert_eq!(config.deny_extensions(), &["html", "js", "css"]);
        assert_eq!(config.lb_algo(), "roundrobin");
        assert_eq!(config.servers(), &["127.0.0.1:3001", "127.0.0.1:3002"]);
        assert_eq!(config.root(), ".");
        assert_eq!(config.deny_directories(), &[".git", ".svn"]);
        assert_eq!(config.allow_directories(), &["assets/images", "assets/css"]);

        Ok(())
    }
}
