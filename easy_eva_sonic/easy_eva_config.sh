#!/data/data/com.termux/files/usr/bin/bash
# Easy-Eva-Sonic Security Configuration

echo "🔧 EASY-EVA SECURITY KONFIGURATION"
echo "=================================="

cd ~/easy_eva_sonic

# Master-Hash anzeigen
echo ""
echo "🔐 MASTER-HASH:"
python3 -c "
import json
try:
    with open('blockchain_master_hash.json', 'r') as f:
        data = json.load(f)
    hash = data.get('master_hash', 'NOT FOUND')
    blocks = data.get('block_count', 0)
    print(f'Hash:    {hash[:32]}...')
    print(f'Blöcke:  {blocks:,}')
    print(f'Nodes:   {blocks * 1000:,}')
    print(f'Datum:   {data.get(\"timestamp\", \"Unknown\")}')
except:
    print('blockchain_master_hash.json nicht gefunden')
"

# Daily Check einrichten
echo ""
echo "📅 TÄGLICHE ÜBERPRÜFUNG EINRICHTEN:"
echo "pkg install cronie -y"
echo "crond"
echo ""
echo "Dann in crontab eintragen:"
echo "0 2 * * * cd ~/easy_eva_sonic && python daily_security_check.py >> daily.log 2>&1"

# Security Status
echo ""
echo "📊 SECURITY STATUS:"
if [ -f "security_monitor.log" ]; then
    echo "✅ Monitoring Log vorhanden"
    tail -3 security_monitor.log
else
    echo "⚠️  Kein Monitoring Log"
fi

# Backup Status
echo ""
echo "💾 BACKUP STATUS:"
backups=$(ls -1 emergency_backup_*.db 2>/dev/null | wc -l)
echo "Emergency Backups: $backups"
if [ $backups -gt 0 ]; then
    ls -lt emergency_backup_*.db 2>/dev/null | head -3
fi

echo ""
echo "🚀 SYSTEM BEREIT!"
echo "Nutze: python eva_blockchain_security.py für das Menü"
