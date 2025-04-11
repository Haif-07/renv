
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

use regex::Regex;

// const ZSHENV: &str = "~/.zshenv";
// const ZPROFILE: &str = "~/.zprofile";
// const ZSHRC: &str = "~/.zshrc";
// const ZLOGIN: &str = "~/.zlogin";
// const ZLOGOUT: &str = "~/.zlogout";
// const BASH_PROFILE: &str = "~/.bash_profile";
// const BASH_RC: &str = "~/.bashrc";
// const BASH_LOGIN: &str = "~/.bash_login";
// const BASH_LOGOUT: &str = "~/.bash_logout";
// const PROFILE: &str = "~/.profile";


// 通用配置解析器
pub struct ShellConfig {
    pub path: String,
    pub  env_vars: HashMap<u64, EnvVariable>,
    pub path_line:u64,
    pub all_lines:u64,
}

pub struct EnvVariable {
    pub key: String,
    pub value: Vec<String>,
}

impl ShellConfig {
    pub  fn new(path: &str) -> Self {
        ShellConfig {
            path: path.to_string(),
            env_vars: HashMap::new(),
            path_line:0,
            all_lines:0,
        }
    }
    pub fn load(&mut self) -> io::Result<()> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        // 收集所有行到 Vec 中，这样可以多次使用
        let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;
        self.all_lines = lines.len() as u64;
       
       
        for (index, line) in lines.iter().enumerate() {
            self.parse_line(index as u64 + 1, &line);
        }
        Ok(())
    }
    pub fn update_env(&mut self, index: &u64, env: &EnvVariable) -> io::Result<()> {

      
        // 更新内存中的数据
        self.env_vars.insert(*index, EnvVariable {
            key: env.key.to_string(),
            value: env.value.clone(),
        });

             
        Ok(())
    }
    pub fn add_env(&mut self, env: &EnvVariable) -> io::Result<()> {
        // 如果已经有这个环境变量，则提示错误
        for ele in &self.env_vars {
            if ele.1.key == env.key {
                return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Environment variable already exists"));
            }
        }   

        self.all_lines += 1;
        print!("{}",self.all_lines);
        // 更新内存中的数据
        self.env_vars.insert(self.all_lines, EnvVariable {
            key: env.key.to_string(),
            value: env.value.clone(),
        });
        
        Ok(())
    }
    pub fn remove_env(&mut self, index: &u64) -> io::Result<()> {
        // 如果没有这个环境变量，则提示错误
        if !self.env_vars.contains_key(index) {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Environment variable not found"));
        }
        // 更新内存中的数据
        self.env_vars.remove(index);
        Ok(())
    }

    pub fn save(&self) -> io::Result<()> {
        // 读取所有行
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
        
        // 更新需要修改的行
        for (index, env) in &self.env_vars {
           
                let nowindex=*index as usize - 1;
                if nowindex >= lines.len() {
                    lines.push(format!("export {}=\"{}\"", env.key, env.value.join(":")));
                    continue;
                }

                lines[nowindex] = format!("export {}=\"{}\"", env.key, env.value.join(":"));
            
        }
        
        // 将所有内容写回文件
        let content = lines.join("\n") + "\n";
        fs::write(&self.path, content)?;
        
        Ok(())
    }
    fn parse_line(&mut self, index:u64,line: &str) {
        let line = line.trim();
        let env_re = Regex::new(r#"^export\s+([a-zA-Z_]+)=['"]?(.*?)['"]?$"#).unwrap();
        if let Some(caps) = env_re.captures(line) {
            let key = caps[1].to_string();
            let value_str = caps[2].to_string();
            
            let value = if value_str.contains("http://") || value_str.contains("https://") {
                vec![value_str]
            } else {
                value_str.split(':')
                .map(|s| s.to_string())
                .collect()
            };

            let env = EnvVariable { key: key.clone(), value };
            if key == "PATH" {
                self.path_line = index;
            }
            self.env_vars.insert(index, env);
        }
    }
    
}

