#!/usr/bin/env python3
"""
Generate Software Bill of Materials (SBOM) for hybrid-kem
"""

import json
import subprocess
import sys
from datetime import datetime

def get_system_info():
    """Get system and dependency information"""
    info = {
        "timestamp": datetime.now().isoformat(),
        "project": "hybrid-kem",
        "version": "0.1.0"
    }
    
    # Get OpenSSL version
    try:
        openssl_result = subprocess.run(["openssl", "version"], 
                                       capture_output=True, text=True)
        info["openssl"] = openssl_result.stdout.strip()
    except:
        info["openssl"] = "Not found"
    
    # Get CMake version
    try:
        cmake_result = subprocess.run(["cmake", "--version"], 
                                     capture_output=True, text=True)
        lines = cmake_result.stdout.split('\n')
        info["cmake"] = lines[0] if lines else "Not found"
    except:
        info["cmake"] = "Not found"
    
    # Get compiler info
    try:
        cc_result = subprocess.run(["clang", "--version"], 
                                  capture_output=True, text=True)
        info["compiler"] = cc_result.stdout.split('\n')[0]
    except:
        try:
            cc_result = subprocess.run(["gcc", "--version"], 
                                      capture_output=True, text=True)
            info["compiler"] = cc_result.stdout.split('\n')[0]
        except:
            info["compiler"] = "Unknown"
    
    return info

def generate_sbom():
    """Generate SBOM in CycloneDX format"""
    sbom = {
        "bomFormat": "CycloneDX",
        "specVersion": "1.4",
        "version": 1,
        "metadata": {
            "timestamp": datetime.now().isoformat(),
            "tools": [
                {
                    "vendor": "hybrid-kem",
                    "name": "sbom-generator",
                    "version": "1.0"
                }
            ],
            "component": {
                "type": "library",
                "bom-ref": "pkg:generic/hybrid-kem@0.1.0",
                "name": "hybrid-kem",
                "version": "0.1.0",
                "description": "Hybrid Post-Quantum KEM Library"
            }
        },
        "components": [
            {
                "type": "library",
                "bom-ref": "pkg:generic/openssl",
                "name": "openssl",
                "version": "3.6.0",
                "description": "Cryptography and SSL/TLS Toolkit"
            },
            {
                "type": "framework",
                "bom-ref": "pkg:generic/cmake",
                "name": "cmake",
                "version": "3.22.1",
                "description": "Cross-platform build system"
            }
        ],
        "dependencies": [
            {
                "ref": "pkg:generic/hybrid-kem@0.1.0",
                "dependsOn": ["pkg:generic/openssl", "pkg:generic/cmake"]
            }
        ]
    }
    
    # Add system info
    system_info = get_system_info()
    sbom["metadata"]["properties"] = [
        {"name": "build:system", "value": json.dumps(system_info)}
    ]
    
    return sbom

def main():
    print("🔍 Generating Software Bill of Materials (SBOM)...")
    
    sbom = generate_sbom()
    
    # Save as JSON
    with open("sbom.json", "w") as f:
        json.dump(sbom, f, indent=2)
    
    print("✅ SBOM generated: sbom.json")
    print(f"   Components: {len(sbom['components'])}")
    print(f"   Dependencies: {len(sbom['dependencies'])}")
    
    # Show quick summary
    print("\n📋 Quick Summary:")
    print(f"   Project: {sbom['metadata']['component']['name']} v{sbom['metadata']['component']['version']}")
    for comp in sbom['components']:
        print(f"   Dependency: {comp['name']} v{comp['version']}")

if __name__ == "__main__":
    main()
