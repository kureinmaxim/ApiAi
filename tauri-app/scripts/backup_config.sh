#!/bin/bash
# Backup Current ApiAi Configuration
# Создает резервную копию текущей конфигурации

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}💾 ApiAi Configuration Backup${NC}"
echo ""

# Determine OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    CONFIG_DIR="$HOME/Library/Application Support/com.apiai.app"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    CONFIG_DIR="$HOME/.local/share/com.apiai.app"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    CONFIG_DIR="$APPDATA/com.apiai.app"
else
    echo -e "${YELLOW}⚠️  Unknown OS: $OSTYPE${NC}"
    exit 1
fi

CONFIG_FILE="$CONFIG_DIR/config.json"
BACKUP_DIR="$HOME/Documents/ApiAi_Backups"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/config_$TIMESTAMP.json"

# Check if config exists
if [ ! -f "$CONFIG_FILE" ]; then
    echo -e "${YELLOW}⚠️  No config file found at:${NC}"
    echo "   $CONFIG_FILE"
    echo ""
    echo "Run the app first to generate the config file."
    exit 1
fi

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Copy config
echo -e "${BLUE}📋 Backing up configuration...${NC}"
cp "$CONFIG_FILE" "$BACKUP_FILE"

echo ""
echo -e "${GREEN}✅ Backup created successfully!${NC}"
echo ""
echo -e "${BLUE}📍 Location: $BACKUP_FILE${NC}"
echo -e "${BLUE}📁 Backup folder: $BACKUP_DIR${NC}"
echo ""
