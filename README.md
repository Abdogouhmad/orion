# WhisperCLI
## Overview

WhisperCLI is a command-line interface tool built with Rust, 
aimed at minimizing the amount of written command line in the terminal. 
It simplifies various tasks such as cloning repositories, 
managing packages, measuring folder sizes, and handling Git operations.

## Features

- Clone: Allows you to clone any repository. Use `whispercli clone --help` for more information.
- List: Lists Pacman or Yay packages that need to be updated. Use -l or --list followed by the package manager name.
- Update: Updates the specified package(s) for Pacman, Yay, or both. Use -u or --update followed by the package manager name.
- Weight: Measures the size of folders. Use -w or --weight.
- Delete: Deletes the logs folder. Use -d or --delete.
- Status: Provides the files that are in a changed Git status. Use -s or --status.
- Commit: Commit and push the changes regardless the branch you are in with option `-c="your commit"`.

## Usage
To use WhisperCLI, follow the general format: 
```bash
whispercli [OPTIONS] [COMMAND].
```

For detailed usage instructions, refer to the help command: whispercli --help.

## Contribution

We welcome contributions from the community. To contribute, please fork the repository, 
make your changes, and submit a pull request.
