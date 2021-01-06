pub enum After {
  Refresh,
  Exit,
}

pub struct KeyBinding {
  pub key: char,
  pub command: String,
  pub confirm: bool,
  pub after: After,
  pub regex: Option<String>,
}

impl Default for KeyBinding {
  fn default() -> KeyBinding {
    KeyBinding {
      key: ' ',
      command: String::from(""),
      confirm: false,
      after: After::Refresh,
      regex: None,
    }
  }
}

pub struct Profile {
  pub name: String,
  pub registered_commands: Vec<String>,
  pub key_bindings: Vec<KeyBinding>,
  pub lines_to_skip: usize,
}

pub struct Config {
  pub profiles: Vec<Profile>,
}

impl Config {
  pub fn new() -> Config {
    // just doing a dummy one for now
    Config {
      profiles: vec![
        Profile {
          name: String::from("ls"),
          registered_commands: vec![String::from("ls -1"), String::from("ls -a")],
          key_bindings: vec![
            KeyBinding {
              key: 'd',
              command: String::from("rm $0"),
              confirm: true,
              ..Default::default()
            },
            KeyBinding {
              key: 'o',
              command: String::from("code -r $0"),
              ..Default::default()
            },
            KeyBinding {
              key: 'u',
              command: String::from("cd $0"),
              ..Default::default()
            },
          ],
          lines_to_skip: 0,
        },
        Profile {
          name: String::from("ls -l"),
          registered_commands: vec![String::from("ls -l")],
          key_bindings: vec![
            KeyBinding {
              key: 'd',
              command: String::from("rm $8"),
              confirm: true,
              ..Default::default()
            },
            KeyBinding {
              key: 'o',
              command: String::from("code -r $8"),
              ..Default::default()
            },
            KeyBinding {
              key: 'u',
              command: String::from("cd $8"),
              ..Default::default()
            },
          ],
          lines_to_skip: 0,
        },
        Profile {
          name: String::from("git status --short"),
          registered_commands: vec![String::from("git status --short")],
          key_bindings: vec![
            KeyBinding {
              key: 'A',
              command: String::from("git add $1"),
              ..Default::default()
            },
            KeyBinding {
              key: 'a',
              command: String::from("git reset $1"),
              confirm: true,
              ..Default::default()
            },
            KeyBinding {
              key: 'd',
              command: String::from("rm $1"),
              confirm: true,
              ..Default::default()
            },
          ],
          lines_to_skip: 0,
        },
        Profile {
          name: String::from("git status"),
          registered_commands: vec![String::from("git status")],
          key_bindings: vec![
            KeyBinding {
              key: 'A',
              command: String::from("git add $0"),
              ..Default::default()
            },
            KeyBinding {
              key: 'a',
              command: String::from("git reset $1"),
              confirm: true,
              regex: Some(String::from(".*:\\s+(\\w+)")),
              ..Default::default()
            },
            KeyBinding {
              key: 'd',
              command: String::from("rm $1"),
              confirm: true,
              ..Default::default()
            },
          ],
          lines_to_skip: 0,
        },
        Profile {
          name: String::from("docker ps"),
          registered_commands: vec![String::from("docker ps")],
          key_bindings: vec![
            KeyBinding {
              key: 's',
              command: String::from("docker stop $0"),
              confirm: true,
              ..Default::default()
            },
            KeyBinding {
              key: 'r',
              command: String::from("docker restart $0"),
              confirm: true,
              ..Default::default()
            },
            KeyBinding {
              key: 'd',
              command: String::from("docker kill $0"),
              confirm: true,
              ..Default::default()
            },
          ],
          lines_to_skip: 0,
        },
        Profile {
          name: String::from("git branch"),
          registered_commands: vec![String::from("git branch")],
          key_bindings: vec![KeyBinding {
            key: 'c',
            command: String::from("git checkout $1"),
            ..Default::default()
          }],
          lines_to_skip: 0,
        },
        Profile {
          name: String::from("lsof -iTCP | grep LISTEN"),
          registered_commands: vec![
            String::from("lsof -iTCP | grep LISTEN"),
            String::from("lsof -iTCP"),
          ],
          key_bindings: vec![KeyBinding {
            key: 'd',
            command: String::from("kill -9 $1"),
            confirm: true,
            ..Default::default()
          }],
          lines_to_skip: 0,
        },
      ],
    }
  }
}
