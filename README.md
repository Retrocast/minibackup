# minibackup

Small Rust program I made to backup my files.

## Installation

1. Clone the repository.  
   `git clone https://github.com/Retrocast/minibackup.git && cd minibackup`
2. Build the project.  
   `cargo build --release`
3. Put the binary (`target/release/minibackup`) somewhere like `~/opt/minibackup`.
4. Create config file in same directory as binary. You can use example from below to make the config file.
5. When you want to make a backup, just run the binary in that directory.

## Configuration

Program loads configuration from `config.toml` in current directory.
Example config file:

```toml
[archive]
# If set to true, you will be prompted to enter a password which will be used to encrypt the archive. Defaults to false.
encrypt = true
# Path to output file. If it doesn't exist, it'll be created. If it exists, it'll be overwritten. Defaults to "backup.zip".
dest = "/mnt/hdd/backup.zip"

# You can specify as many sources as you want.
[[sources]]
# Can be "command", "directory" or "file".
# "command" type executes a command and saves its stdout output.
type = "command"
# Command itself. As of now, it is executed via bash -c, so it may use $VARIABLES and other Bash syntax.
# Step will fail (and not save command output) if it returns nonzero exit code.
cmd = "pacman -Qeq"
# Where to put command output in archive. Directories in path are supported too.
dest = "pacman/explicit.txt"

[[sources]]
# Recursively archives a directory.
type = "directory"
# Directory path. As of now, it doesn't support things like ~ and it must be a absolute/relative path.
path = "/home/user/Documents"
# Where to put directory in archive. Directories in path are supported too. Optional, defaults to base folder name in archive root (e.g. "/Documents" in this example).
dest = "docs"
# Whether to respect .gitignore files when traversing directory (e.g. do not add gitignored files to archive). Optional, defaults to true.
respect_gitignore = true
# Whether to skip hidden files (ones starting with dot). Optional, defaults to false.
skip_hidden = false
# Gitignore-like list of files/directories to exclude. Due to how `ignore` crate works, inversion with ! is not available. Optional.
exclude = ["*.pdf"]
# Max file size in bytes.
max_file_size = 10e6 # 10 MB / 9.5 MiB

[[sources]]
# Archives a single file.
type = "file"
# File path. As of now, it doesn't support things like ~ and it must be a absolute/relative path.
path = "/home/user/Desktop/secret.txt"
# Where to put file in archive. Directories in path are supported too. Optional, defaults to base folder name in archive root (e.g. "/secret.txt" in this example).
dest = "important/secret.txt"
```
