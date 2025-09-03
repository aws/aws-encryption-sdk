#!/usr/bin/env python3
"""
Configuration module for ESDK Python benchmark
"""

import yaml


def load_config(config_path: str):
    """Load test configuration from YAML file"""
    try:
        with open(config_path, "r") as f:
            return yaml.safe_load(f)
    except FileNotFoundError:
        raise FileNotFoundError(f"Config file not found: {config_path}")
    except Exception as e:
        raise RuntimeError(f"Failed to parse config file: {e}")
