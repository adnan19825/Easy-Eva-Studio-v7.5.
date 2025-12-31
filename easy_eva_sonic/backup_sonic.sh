#!/bin/bash
# Easy-Eva Sonic Backup Skript
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="backups/v10.8.10_$TIMESTAMP"

echo "📦 Erstelle Backup in $BACKUP_DIR..."
mkdir -p "$BACKUP_DIR"

# Code und Konfiguration sichern
cp src/main.rs "$BACKUP_DIR/"
cp src/comms.rs "$BACKUP_DIR/"
cp Cargo.toml "$BACKUP_DIR/"
cp .env "$BACKUP_DIR/" 2>/dev/null

# Die mühsam gelernte Baseline sichern
if [ -f "baseline.json" ]; then
    cp baseline.json "$BACKUP_DIR/"
    echo "✅ Baseline gesichert."
fi

echo "🚀 Backup abgeschlossen!"

