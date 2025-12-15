#!/bin/bash
echo "Creating backup..."
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
BACKUP_FILE="$HOME/hybrid-kem-backups/hybrid-kem_${TIMESTAMP}.tar.gz"
mkdir -p "$HOME/hybrid-kem-backups"
tar -czf "$BACKUP_FILE" -C "$HOME" hybrid-kem 2>/dev/null
echo "Backup created: $BACKUP_FILE"
