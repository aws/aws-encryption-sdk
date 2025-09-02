// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package benchmark

import (
	"fmt"
	"os"

	"gopkg.in/yaml.v3"
)

// TestConfig represents the configuration for benchmark tests
type TestConfig struct {
	DataSizes struct {
		Small  []int `yaml:"small"`
		Medium []int `yaml:"medium"`
		Large  []int `yaml:"large"`
	} `yaml:"data_sizes"`
	Iterations struct {
		Warmup      int `yaml:"warmup"`
		Measurement int `yaml:"measurement"`
	} `yaml:"iterations"`
	ConcurrencyLevels []int        `yaml:"concurrency_levels"`
	QuickConfig       *QuickConfig `yaml:"quick_config"`
}

// QuickConfig represents the quick test configuration
type QuickConfig struct {
	DataSizes struct {
		Small []int `yaml:"small"`
	} `yaml:"data_sizes"`
	Iterations struct {
		Warmup      int `yaml:"warmup"`
		Measurement int `yaml:"measurement"`
	} `yaml:"iterations"`
	ConcurrencyLevels []int    `yaml:"concurrency_levels"`
	TestTypes         []string `yaml:"test_types"`
}

// LoadConfig loads the test configuration from YAML file
func LoadConfig(configPath string) (TestConfig, error) {
	var config TestConfig

	if _, err := os.Stat(configPath); os.IsNotExist(err) {
		return config, fmt.Errorf("config file not found: %s", configPath)
	}

	data, err := os.ReadFile(configPath)
	if err != nil {
		return config, fmt.Errorf("failed to read config file: %w", err)
	}

	if err := yaml.Unmarshal(data, &config); err != nil {
		return config, fmt.Errorf("failed to parse config file: %w", err)
	}

	return config, nil
}
