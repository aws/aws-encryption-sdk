#!/usr/bin/env python3
"""
Results module for ESDK Python benchmark
"""

import json
import multiprocessing
import sys
import time
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import List

import psutil


@dataclass
class BenchmarkResult:
    """Container for benchmark results"""

    test_name: str
    language: str = "python"
    data_size: int = 0
    concurrency: int = 1
    encrypt_latency_ms: float = 0.0
    decrypt_latency_ms: float = 0.0
    end_to_end_latency_ms: float = 0.0
    ops_per_second: float = 0.0
    bytes_per_second: float = 0.0
    peak_memory_mb: float = 0.0
    memory_efficiency_ratio: float = 0.0
    p50_latency: float = 0.0
    p95_latency: float = 0.0
    p99_latency: float = 0.0
    timestamp: str = ""
    python_version: str = ""
    cpu_count: int = 0
    total_memory_gb: float = 0.0

    def __post_init__(self):
        self.timestamp = self.timestamp or time.strftime("%Y-%m-%d %H:%M:%S")
        self.python_version = self.python_version or self._get_python_version()
        self.cpu_count = self.cpu_count or multiprocessing.cpu_count()
        self.total_memory_gb = self.total_memory_gb or self._get_total_memory()

    def _get_python_version(self):
        """Get Python version string"""
        return (
            f"{sys.version_info.major}.{sys.version_info.minor}."
            f"{sys.version_info.micro}"
        )

    def _get_total_memory(self):
        """Get total system memory in GB"""
        return psutil.virtual_memory().total / (1024**3)


def save_results(results: List[BenchmarkResult], output_path: str):
    """Save benchmark results to JSON file"""
    output_file = Path(output_path)
    output_file.parent.mkdir(parents=True, exist_ok=True)

    metadata = _create_metadata(results)
    results_data = {
        "metadata": metadata,
        "results": [asdict(result) for result in results],
    }

    with open(output_file, "w") as f:
        json.dump(results_data, f, indent=2)


def _create_metadata(results: List[BenchmarkResult]):
    """Create metadata for results file"""
    metadata = {
        "language": "python",
        "timestamp": time.strftime("%Y-%m-%d %H:%M:%S"),
        "total_tests": len(results),
    }

    if results:
        metadata.update(
            {
                "python_version": results[0].python_version,
                "cpu_count": results[0].cpu_count,
                "total_memory_gb": results[0].total_memory_gb,
            }
        )

    return metadata
