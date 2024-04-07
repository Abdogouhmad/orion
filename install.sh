#!/bin/bash

GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

printcl() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

install_orion() {
    printcl "${YELLOW}" "Installing The CLI into /usr/bin..."
    sudo curl -L https://github.com/div-styl/orion/releases/download/2.2.4/orion -o /usr/bin/orion
    sudo chmod +x /usr/bin/orion
    printcl "${GREEN}" "Installed Successfully! Run the CLI with the command: orion -h"
}

uninstall_orion() {
    printcl "${YELLOW}" "Uninstalling The CLI from /usr/bin..."
    sudo rm /usr/bin/orion
    printcl "${GREEN}" "Uninstalled Successfully!"
}

printcl "${YELLOW}" "Choose an action:"
printcl "${YELLOW}" "1: Install"
printcl "${YELLOW}" "2: Uninstall"
read -p "Enter your choice: " choice

case "$choice" in
    1)
        install_orion
        ;;
    2)
        uninstall_orion
        ;;
    *)
        printcl "${YELLOW}" "Invalid choice. Please choose 1 to install or 2 to uninstall."
        exit 1
        ;;
esac

exit 0
