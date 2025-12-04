#!/bin/bash
# Restore ApiAi Configuration
# Восстанавливает конфигурацию из шаблона в app data директорию

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔧 ApiAi Configuration Restore${NC}"
echo ""

# Determine OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    CONFIG_DIR="$HOME/Library/Application Support/com.apiai.app"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    CONFIG_DIR="$HOME/.local/share/com.apiai.app"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    # Windows
    CONFIG_DIR="$APPDATA/com.apiai.app"
else
    echo -e "${YELLOW}⚠️  Unknown OS: $OSTYPE${NC}"
    echo "Please manually copy config.json.template to your app data directory"
    exit 1
fi

TEMPLATE_FILE="config_templates/config.json.template"
CONFIG_FILE="$CONFIG_DIR/config.json"

# Check if template exists
if [ ! -f "$TEMPLATE_FILE" ]; then
    echo -e "${YELLOW}⚠️  Template not found: $TEMPLATE_FILE${NC}"
    exit 1
fi

# Create config directory if it doesn't exist
if [ ! -d "$CONFIG_DIR" ]; then
    echo -e "${BLUE}📁 Creating config directory...${NC}"
    mkdir -p "$CONFIG_DIR"
fi

# Check if config already exists
if [ -f "$CONFIG_FILE" ]; then
    echo -e "${YELLOW}⚠️  Config file already exists:${NC}"
    echo "   $CONFIG_FILE"
    echo ""
    read -p "Do you want to overwrite it? (y/N): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${BLUE}ℹ️  Restoration cancelled${NC}"
        exit 0
    fi
    
    # Backup existing config
    BACKUP_FILE="$CONFIG_FILE.backup.$(date +%Y%m%d_%H%M%S)"
    echo -e "${BLUE}💾 Creating backup: $(basename $BACKUP_FILE)${NC}"
    cp "$CONFIG_FILE" "$BACKUP_FILE"
fi

# Copy template to config location
echo -e "${BLUE}📋 Restoring configuration...${NC}"
cp "$TEMPLATE_FILE" "$CONFIG_FILE"

echo ""
echo -e "${GREEN}✅ Configuration restored successfully!${NC}"
echo ""
echo -e "${BLUE}📍 Location: $CONFIG_FILE${NC}"
echo ""
echo -e "${BLUE}ℹ️  Next steps:${NC}"
echo "   1. Launch ApiAi"
echo "   2. Your settings should be loaded automatically"
echo "   3. If using development mode: npm run tauri dev"
echo ""
