import docker
import logging

client = docker.from_env()

def isolate_container(container_id):
    try:
        container = client.containers.get(container_id)
        
        # 1. Container pausieren (stoppt CPU-Aktivität, bewahrt RAM für Forensik)
        container.pause()
        logging.info(f"Container {container_id} wurde pausiert.")

        # 2. Netzwerke trennen
        networks = container.attrs['NetworkSettings']['Networks']
        for net_name in networks.keys():
            network = client.networks.get(net_name)
            network.disconnect(container)
            logging.info(f"Container von Netzwerk {net_name} getrennt.")

        # 3. In Quarantäne-Netzwerk schieben (optional, für Analyse)
        # quarantine_net = client.networks.get("quarantine_vlan")
        # quarantine_net.connect(container)

        print(f"[SUCCESS] Node {container_id} ist nun isoliert.")
        
    except Exception as e:
        logging.error(f"Fehler bei der Isolation: {e}")

if __name__ == "__main__":
    # Beispiel-Aufruf durch deinen Monitor
    isolate_container("deine_container_id_oder_name")

