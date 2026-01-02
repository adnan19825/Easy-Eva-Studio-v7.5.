#!/data/data/com.termux/files/usr/bin/python3
"""
EASY-EVA-SONIC SECURITY MONITOR V2.0
Real-Time Intrusion Detection System
"""

import os
import hashlib
import json
import sys
import time
import threading
from datetime import datetime
from pathlib import Path
import sqlite3

print("""
╔══════════════════════════════════════════════════════════╗
║      EASY-EVA-SONIC SECURITY MONITOR V2.0                ║
║      Real-Time Intrusion Detection System                ║
╚══════════════════════════════════════════════════════════╝
""")

class RealTimeSecurityMonitor:
    def __init__(self, project_root="."):
        self.project_root = Path(project_root)
        self.suspicious_events = []
        self.file_hashes = {}
        self.running = False
        
    def calculate_robust_hash(self, filepath):
        """Berechnet Hash durch Sampling"""
        try:
            size = os.path.getsize(filepath)
            
            with open(filepath, 'rb') as f:
                # Sample aus verschiedenen Bereichen
                samples = []
                
                # Anfang
                samples.append(f.read(1024))
                
                # Mitte (wenn groß genug)
                if size > 5000:
                    f.seek(size // 2)
                    samples.append(f.read(1024))
                
                # Ende
                if size > 1024:
                    f.seek(max(0, size - 1024))
                    samples.append(f.read(1024))
                
                combined = b''.join(samples)
                return hashlib.sha256(combined).hexdigest()
                
        except Exception as e:
            return f"ERROR: {e}"
    
    def initialize_baseline(self):
        """Erstellt Baseline aller wichtigen Dateien"""
        print("🔐 Initialisiere Security Baseline...")
        
        critical_files = [
            'easy_eva_v7_5.db',
            'easy_eva_sonic.db',
            'config.json',
            'main.py',
            'Cargo.toml'
        ]
        
        for file in critical_files:
            if os.path.exists(file):
                self.file_hashes[file] = self.calculate_robust_hash(file)
                print(f"   • {file}: {self.file_hashes[file][:16]}...")
        
        # Speichere Baseline
        with open('security_baseline.json', 'w') as f:
            json.dump({
                'timestamp': datetime.now().isoformat(),
                'hashes': self.file_hashes
            }, f, indent=2)
        
        print(f"✅ Baseline mit {len(self.file_hashes)} Dateien erstellt")
    
    def check_database_integrity(self):
        """Tiefgehende DB-Prüfung"""
        print("\n🗃️  DATENBANK-INTEGRITÄTSCHECK:")
        
        db_files = ['easy_eva_v7_5.db', 'easy_eva_sonic.db']
        
        for db_file in db_files:
            if os.path.exists(db_file):
                size = os.path.getsize(db_file)
                modified = datetime.fromtimestamp(os.path.getmtime(db_file))
                
                print(f"\n📊 {db_file}:")
                print(f"   • Größe: {size:,} Bytes")
                print(f"   • Letzte Änderung: {modified}")
                
                # Quick Hash
                current_hash = self.calculate_robust_hash(db_file)
                print(f"   • Power-Hash: {current_hash[:32]}...")
                
                # SQLite Format prüfen
                try:
                    with open(db_file, 'rb') as f:
                        header = f.read(16)
                        if header.startswith(b'SQLite format 3'):
                            print("   ✅ Valides SQLite-Format")
                        else:
                            print("   ⚠️  Kein SQLite-Format!")
                except:
                    print("   ❌ Kann Datei nicht lesen")
                
                # Tabellen prüfen (wenn möglich)
                try:
                    conn = sqlite3.connect(db_file)
                    cursor = conn.cursor()
                    cursor.execute("SELECT name FROM sqlite_master WHERE type='table';")
                    tables = cursor.fetchall()
                    conn.close()
                    
                    print(f"   • Tabellen: {len(tables)}")
                    if tables:
                        print(f"   • Erste Tabelle: {tables[0][0]}")
                except:
                    print("   • Kann Tabellen nicht auslesen")
                
            else:
                print(f"   ❌ {db_file}: Nicht gefunden")
    
    def quick_scan(self):
        """Schneller System-Scan"""
        print("\n🔍 QUICK SECURITY SCAN:")
        
        critical_files = [
            'easy_eva_v7_5.db',
            'easy_eva_sonic.db',
            'config.json',
            'main.py',
            'Cargo.toml',
            'src/main.rs'
        ]
        
        all_ok = True
        for file in critical_files:
            if os.path.exists(file):
                current_hash = self.calculate_robust_hash(file)
                
                if file in self.file_hashes:
                    if current_hash == self.file_hashes[file]:
                        print(f"✅ {file}: OK")
                    else:
                        print(f"❌ {file}: VERÄNDERT!")
                        all_ok = False
                else:
                    print(f"⚠️  {file}: Neu (keine Baseline)")
            else:
                print(f"📭 {file}: Nicht gefunden")
        
        return all_ok
    
    def generate_report(self):
        """Erstellt Security Report"""
        report = {
            'timestamp': datetime.now().isoformat(),
            'system': {
                'python': sys.version,
                'platform': sys.platform,
                'cwd': os.getcwd()
            },
            'files_scanned': len(self.file_hashes),
            'current_hashes': {},
            'recommendations': [
                "1. Regelmäßig Backups erstellen",
                "2. Hashes mit GitHub vergleichen",
                "3. Dateiänderungen protokollieren",
                "4. Externe Sicherungen anlegen"
            ]
        }
        
        # Aktuelle Hashes sammeln
        for file in self.file_hashes.keys():
            if os.path.exists(file):
                report['current_hashes'][file] = self.calculate_robust_hash(file)
        
        # Report speichern
        report_file = f"security_report_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(report_file, 'w') as f:
            json.dump(report, f, indent=2)
        
        print(f"\n📊 Security Report gespeichert: {report_file}")
        return report
    
    def start_file_monitoring(self):
        """Startet einfache Datei-Überwachung"""
        print("\n👁️  STARTE DATEI-ÜBERWACHUNG")
        print("   Strg+C zum Stoppen")
        print("-" * 50)
        
        initial_hashes = {}
        for file in self.file_hashes.keys():
            if os.path.exists(file):
                initial_hashes[file] = self.calculate_robust_hash(file)
        
        try:
            while True:
                time.sleep(5)  # Alle 5 Sekunden prüfen
                
                for file, initial_hash in initial_hashes.items():
                    if os.path.exists(file):
                        current_hash = self.calculate_robust_hash(file)
                        
                        if current_hash != initial_hash:
                            print(f"\n🚨 ALERT: {file} wurde verändert!")
                            print(f"   Vorher: {initial_hash[:16]}...")
                            print(f"   Jetzt:  {current_hash[:16]}...")
                            
                            # Backup der veränderten Datei
                            backup_name = f"{file}.backup_{int(time.time())}"
                            import shutil
                            shutil.copy2(file, backup_name)
                            print(f"   💾 Backup: {backup_name}")
                            
                            # Hash aktualisieren
                            initial_hashes[file] = current_hash
                
                print(".", end="", flush=True)
                
        except KeyboardInterrupt:
            print("\n\n🛑 Überwachung gestoppt")

def main():
    """Hauptfunktion"""
    monitor = RealTimeSecurityMonitor()
    
    # Baseline laden oder erstellen
    if os.path.exists('security_baseline.json'):
        try:
            with open('security_baseline.json', 'r') as f:
                baseline = json.load(f)
                monitor.file_hashes = baseline.get('hashes', {})
            print(f"📖 Baseline geladen vom {baseline.get('timestamp', 'Unbekannt')}")
        except:
            print("⚠️  Baseline konnte nicht geladen werden, erstelle neu...")
            monitor.initialize_baseline()
    else:
        monitor.initialize_baseline()
    
    print("\n" + "="*50)
    print("1. Datenbank-Integrität prüfen")
    print("2. Quick Security Scan")
    print("3. Datei-Überwachung starten")
    print("4. Security Report erstellen")
    print("5. Neue Baseline erstellen")
    print("="*50)
    
    try:
        choice = input("\n🎯 Auswahl (1-5): ").strip()
        
        if choice == '1':
            monitor.check_database_integrity()
        elif choice == '2':
            if monitor.quick_scan():
                print("\n✅ Alle Dateien sind sicher!")
            else:
                print("\n⚠️  Einige Dateien wurden verändert!")
        elif choice == '3':
            monitor.start_file_monitoring()
        elif choice == '4':
            monitor.generate_report()
        elif choice == '5':
            monitor.initialize_baseline()
        else:
            print("❌ Ungültige Auswahl")
            
    except KeyboardInterrupt:
        print("\n👋 Beendet")
    except Exception as e:
        print(f"\n❌ Fehler: {e}")

if __name__ == "__main__":
    main()
