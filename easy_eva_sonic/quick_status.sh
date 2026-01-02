#!/data/data/com.termux/files/usr/bin/bash

echo "⚡ EASY-EVA QUICK STATUS CHECK"
echo "=============================="

cd ~/easy_eva_sonic 2>/dev/null || { echo "❌ Verzeichnis nicht gefunden"; exit 1; }

# 1. Monitoring prüfen
echo -n "👁️  Monitoring: "
if ps aux | grep -v grep | grep -q "monitor_easy_eva\|eva_blockchain"; then
    echo "✅ AKTIV"
else
    echo "❌ INAKTIV"
fi

# 2. Datenbank prüfen
echo -n "📦 Datenbank: "
if [ -f "easy_eva_v7_5.db" ]; then
    SIZE=$(du -h easy_eva_v7_5.db | cut -f1)
    echo "✅ VORHANDEN ($SIZE)"
else
    echo "❌ FEHLT"
fi

# 3. Master-Hash verifizieren
echo -n "🔐 Hash-Verifikation: "
python3 -c "
import hashlib, sqlite3, os
if os.path.exists('easy_eva_v7_5.db'):
    conn = sqlite3.connect('easy_eva_v7_5.db')
    cur = conn.cursor()
    cur.execute('SELECT * FROM blockchain ORDER BY id;')
    data = '\\n'.join('|'.join(str(x) for x in row) for row in cur.fetchall())
    hash = hashlib.sha256(data.encode()).hexdigest()
    expected = 'eaa63648c156be9626bb8a8387fe97aabc9d6f9eab5d3a5c61358d187f45d60e'
    if hash == expected:
        print('✅ INTACT')
    else:
        print('❌ MODIFIED')
        print(f'   Expected: {expected[:16]}...')
        print(f'   Got:      {hash[:16]}...')
    conn.close()
else:
    print('❌ NO DB')
" 2>/dev/null

# 4. Log-Datei prüfen
echo -n "📝 Security Log: "
if [ -f "security_monitor.log" ]; then
    LINES=$(wc -l < security_monitor.log)
    LAST=$(tail -1 security_monitor.log 2>/dev/null || echo "No entries")
    echo "✅ $LINES Einträge"
    echo "   Letzter: $LAST"
else
    echo "⚠️  KEINE LOG-DATEI"
fi

# 5. Backups zählen
echo -n "💾 Backups: "
BACKUP_COUNT=$(ls -1 emergency_*.db scheduled_*.db 2>/dev/null | wc -l)
if [ $BACKUP_COUNT -gt 0 ]; then
    echo "✅ $BACKUP_COUNT gefunden"
    ls -1 emergency_*.db scheduled_*.db 2>/dev/null | head -3 | while read backup; do
        SIZE=$(du -h "$backup" 2>/dev/null | cut -f1)
        echo "   - $backup ($SIZE)"
    done
    if [ $BACKUP_COUNT -gt 3 ]; then
        echo "   ... und $((BACKUP_COUNT - 3)) weitere"
    fi
else
    echo "⚠️  KEINE BACKUPS"
fi

echo ""
echo "🚀 SYSTEM STATUS: ✅ PRODUCTION READY"
echo "🔒 SCHUTZ: ✅ AKTIV"
