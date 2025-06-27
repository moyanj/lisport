import requests

IANA_URL = "https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.csv"
LOCAL_FILE = "raw_services.list"
OUTPUT_FILE = "src/services.list"

# Initialize a dictionary to store service for each port/proto
service_map = {}

# --- Step 1: Load IANA Data ---
try:
    response = requests.get(IANA_URL)
    response.raise_for_status()
    lines = response.text.splitlines()

    for line in lines:
        if line.startswith("Service Name"):
            continue  # Skip header
        parts = line.split(",")
        if len(parts) < 4:
            continue
        name = parts[0].strip()
        port = parts[1].strip()
        proto = parts[2].strip()
        desc = parts[3].strip().lower()

        if not name or not port.isdigit() or proto not in ["tcp", "udp"]:
            continue
        if desc in ["reserved", "unassigned"]:
            continue

        key = (port, proto)
        service_map[key] = name
except Exception as e:
    print("Error fetching IANA data:", e)

# --- Step 2: Override with Local Data ---
try:
    with open(LOCAL_FILE, "r") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            parts = line.split()
            if len(parts) != 2:
                continue
            name, port_proto = parts
            if "/" not in port_proto:
                continue
            port, proto = port_proto.split("/")
            if not port.isdigit() or proto not in ["tcp", "udp"]:
                continue

            key = (port, proto)
            service_map[key] = name
except FileNotFoundError:
    print(f"Local file {LOCAL_FILE} not found. Using only IANA data.")

# --- Step 3: Generate all port/proto combinations from 0 to 65535 ---
with open(OUTPUT_FILE, "w") as f:
    for port in range(0, 65536):
        port_str = str(port)
        tcp_key = (port_str, "tcp")
        udp_key = (port_str, "udp")
        tcp_name = service_map.get(tcp_key)
        udp_name = service_map.get(udp_key)

        if tcp_name is not None:
            f.write(f"{tcp_name} {port_str}/tcp\n")

        if udp_name is not None:
            f.write(f"{udp_name} {port_str}/udp\n")


print(
    f"Service list generated: {len(service_map)} entries applied, others are 'unknown'."
)
print(f"Output saved to {OUTPUT_FILE}")
