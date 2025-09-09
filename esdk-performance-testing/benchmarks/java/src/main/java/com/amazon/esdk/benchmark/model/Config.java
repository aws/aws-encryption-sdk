// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package com.amazon.esdk.benchmark.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.util.List;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public final class Config {

  private static final Logger logger = LoggerFactory.getLogger(Config.class);

  @JsonProperty("data_sizes")
  public DataSizes dataSizes;

  @JsonProperty("iterations")
  public Iterations iterations;

  @JsonProperty("concurrency_levels")
  public List<Integer> concurrencyLevels;

  @JsonProperty("quick_config")
  public QuickConfig quickConfig;

  /** Load test configuration from YAML file */
  public static Config loadConfig(String configPath) throws IOException {
    ObjectMapper mapper = new ObjectMapper(new YAMLFactory());
    File configFile = new File(configPath);

    if (!configFile.exists()) {
      System.err.println("Config file not found, using default configuration");
      throw new FileNotFoundException(configPath);
    }

    return mapper.readValue(configFile, Config.class);
  }

  /** Adjust configuration for quick test */
  public void adjustForQuickTest() {
    this.iterations = quickConfig.iterations;

    this.dataSizes = quickConfig.dataSizes;

    this.concurrencyLevels = quickConfig.concurrencyLevels;
  }

  public static final class DataSizes {

    @JsonProperty("small")
    public List<Integer> small;

    @JsonProperty("medium")
    public List<Integer> medium;

    @JsonProperty("large")
    public List<Integer> large;
  }

  public static final class Iterations {

    @JsonProperty("warmup")
    public int warmup;

    @JsonProperty("measurement")
    public int measurement;
  }

  public static final class QuickConfig {

    @JsonProperty("data_sizes")
    public DataSizes dataSizes;

    @JsonProperty("iterations")
    public Iterations iterations;

    @JsonProperty("concurrency_levels")
    public List<Integer> concurrencyLevels;

    @JsonProperty("test_types")
    public List<String> testTypes;
  }
}
