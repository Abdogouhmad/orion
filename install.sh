#!/usr/bin/bash
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

printcl() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

printcl "${YELLOW}" "Installing The CLI into your /usr/bin..."
sudo curl -L https://github.com/div-styl/orion/releases/download/2.2.1/orion -o /usr/bin/orion
printcl "${GREEN}" "Installed Successfully!"
