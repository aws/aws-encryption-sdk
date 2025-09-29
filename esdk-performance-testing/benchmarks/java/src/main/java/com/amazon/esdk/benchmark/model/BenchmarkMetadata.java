package com.amazon.esdk.benchmark.model;

import com.fasterxml.jackson.annotation.JsonProperty;

public final class BenchmarkMetadata {

  @JsonProperty("language")
  public String language = "java";

  @JsonProperty("timestamp")
  public String timestamp = "";

  @JsonProperty("java_version")
  public String javaVersion = "";

  @JsonProperty("cpu_count")
  public int cpuCount;

  @JsonProperty("total_memory_gb")
  public double totalMemoryGb;

  @JsonProperty("total_tests")
  public int totalTests;
}
