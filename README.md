# minibackup

Small Rust program I made to backup my files.

## Installation

1. Clone the repository.  
   `git clone https://github.com/Retrocast/minibackup.git && cd minibackup`
2. Build the project.  
   `cargo build --release`  
   Path to built binary is `target/release/minibackup`
3. Create a config file. You can use example from below to make the config file.
4. When you want to make a backup, just run the binary and specify path to the config file as first argument (e.g. `./minibackup ~/backup.toml`).

## Configuration

Program loads configuration from file specified in first argument.
If it isn't specified, it'll try to load `config.toml` from current working directory instead.
Example config file and full config format reference:

```toml
[archive]
# Whether encrypt the archive.
# Optional, defaults to false.
encrypt = true
# Password used to encrypt the archive.
# Optional.
# If encrypt is set to true but password isn't specified, you will be prompted to enter password in terminal.
password = "657374726F67656E"
# Path to output file.
# If it doesn't exist, it'll be created. If it exists, it'll be overwritten.
# Optional, defaults to "backup.zip".
dest = "/mnt/hdd/backup.zip"

# You can specify as many sources as you want.
[[sources]]
# Source type can be "command", "directory" or "file".
# "command" type executes a command and archives its stdout output.
type = "command"
# Command to execute.
# As of now, it is executed via bash -c, so it may use $VARIABLES and other Bash syntax.
# Step will fail (and skip archiving) if command has nonzero exit code.
cmd = "pacman -Qeq"
# Where to put command output in archive.
# Directories in path are supported too.
dest = "pacman/explicit.txt"

[[sources]]
# Recursively archives a directory.
type = "directory"
# Directory path.
# As of now, it doesn't support things like ~ and it must be a absolute/relative path.
path = "/home/user/Documents"
# Where to put directory in archive.
# Directories in path are supported too.
# Optional, defaults to base folder name in archive root (e.g. "/Documents" in this example).
dest = "docs"
# Whether to skip files listed in .gitignore.
# .gitignore files in subdirectories are supported too.
# Optional, defaults to true.
respect_gitignore = true
# Whether to skip hidden files (ones starting with a dot).
# Optional, defaults to false.
skip_hidden = false
# Gitignore-like list of files/directories to skip.
# Due to how `ignore` crate works, inversion with ! is not available.
# Optional.
exclude = ["*.pdf"]
# Max file size in bytes.
# Files larger than this limit will be skipped.
# Optional, defaults to 0 (no limit).
max_file_size = 10e6 # 10 MB / 9.5 MiB

[[sources]]
# Archives a single file.
type = "file"
# File path.
# As of now, it doesn't support things like ~ and it must be a absolute/relative path.
path = "/home/user/Desktop/secret.txt"
# Where to put file in archive.
# Directories in path are supported too.
# Optional, defaults to base file name in archive root (e.g. "/secret.txt" in this example).
dest = "important/secret.txt"
```
