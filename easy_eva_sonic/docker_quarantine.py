#!/data/data/com.termux/files/usr/bin/python3
"""
AUTO-QUARANTINE SCRIPT FOR DOCKER ENVIRONMENTS
Trigger: Wird vom Integrity-Monitor bei SHA-256 Mismatch aufgerufen.
Usage: python3 docker_quarantine.py <container_name_or_id> [--pause]
"""
import subprocess
import sys
import logging
from datetime import datetime

# ===== KONFIGURATION =====
LOG_PATH = "/data/data/com.termux/files/usr/var/log/quarantine.log"
QUARANTINE_NETWORK = "quarantine_net"  # Vorher erstellen: docker network create --internal quarantine_net

# ===== LOGGING EINRICHTEN =====
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler(LOG_PATH),
        logging.StreamHandler()
    ]
)

def run_cmd(cmd):
    """Führt Shell-Befehl aus und gibt Ergebnis zurück."""
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True, timeout=30)
        return result.returncode == 0, result.stdout.strip(), result.stderr.strip()
    except subprocess.TimeoutExpired:
        return False, "", "Command timeout"

def ensure_quarantine_network():
    """Stellt sicher, dass das isolierte Quarantäne-Netzwerk existiert."""
    # Prüfen, ob Netzwerk existiert
    success, stdout, stderr = run_cmd(f"docker network ls --filter name=^{QUARANTINE_NETWORK}$ --format '{{{{.Name}}}}'")
    if not success or stdout != QUARANTINE_NETWORK:
        # Netzwerk erstellen (--internal = kein externer Traffic)
        logging.info(f"Quarantäne-Netzwerk '{QUARANTINE_NETWORK}' wird erstellt...")
        success, stdout, stderr = run_cmd(f"docker network create --internal {QUARANTINE_NETWORK}")
        if success:
            logging.info(f"  -> Netzwerk '{QUARANTINE_NETWORK}' erstellt.")
        else:
            logging.error(f"  -> FEHLER beim Erstellen des Netzwerks: {stderr}")
            return False
    return True

def isolate_container(container_id):
    """Isoliert einen Docker-Container im Quarantäne-Netzwerk."""
    # 1. Container in Quarantäne-Netzwerk verbinden (vorher alle anderen Netzwerke trennen)
    logging.info(f"Isoliere Container {container_id}...")

    # Alle aktuellen Netzwerke des Containers abrufen
    success, stdout, stderr = run_cmd(f"docker inspect {container_id} --format='{{{{range $net, $conf := .NetworkSettings.Networks}}}} {{{{printf \"%s\n\" $net}}}} {{{{end}}}}'")
    if success and stdout:
        networks = stdout.split()
        for net in networks:
            if net != QUARANTINE_NETWORK:
                run_cmd(f"docker network disconnect -f {net} {container_id}")
                logging.info(f"  -> Vom Netzwerk '{net}' getrennt.")

    # Mit Quarantäne-Netzwerk verbinden
    success, stdout, stderr = run_cmd(f"docker network connect {QUARANTINE_NETWORK} {container_id}")
    if success:
        logging.info(f"  -> Mit Quarantäne-Netzwerk '{QUARANTINE_NETWORK}' verbunden.")
        return True
    else:
        logging.error(f"  -> FEHLER beim Verbinden mit Quarantäne-Netzwerk: {stderr}")
        return False

def pause_container(container_id):
    """Pausiert den Container (optional)."""
    logging.warning(f"Pausiere Container {container_id}...")
    success, stdout, stderr = run_cmd(f"docker pause {container_id}")
    if success:
        logging.info(f"  -> Container {container_id} pausiert.")
        return True
    else:
        logging.error(f"  -> FEHLER beim Pausieren: {stderr}")
        return False

def main():
    if len(sys.argv) < 2:
        logging.error("Usage: python3 docker_quarantine.py <container_name_or_id> [--pause]")
        sys.exit(1)

    container_id = sys.argv[1]  # Der Integrity-Monitor übergibt den Container-Namen/ID
    should_pause = "--pause" in sys.argv

    logging.info(f"🚨 START Quarantäne-Prozedur für Container: {container_id}")

    # 1. Quarantäne-Netzwerk sicherstellen
    if not ensure_quarantine_network():
        logging.error("Abgebrochen: Quarantäne-Netzwerk konnte nicht bereitgestellt werden.")
        sys.exit(1)

    # 2. Container isolieren
    if not isolate_container(container_id):
        logging.error("Container-Isolierung fehlgeschlagen.")
        sys.exit(1)

    # 3. Optional: Container pausieren
    if should_pause:
        pause_container(container_id)

    # 4. Status in Foreman/logging System aktualisieren (Beispiel)
    # run_cmd(f"echo 'Container {container_id} quarantined at {datetime.now()}' >> /path/to/foreman/log")

    logging.info(f"✅ ENDE Quarantäne-Prozedur für {container_id}. Container ist isoliert.")
    sys.exit(0)

if __name__ == "__main__":
    main()

