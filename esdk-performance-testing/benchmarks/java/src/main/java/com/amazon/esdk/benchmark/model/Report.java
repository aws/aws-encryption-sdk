package com.amazon.esdk.benchmark.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public final class Report {

  private static final Logger logger = LoggerFactory.getLogger(Report.class);

  @JsonProperty("metadata")
  public BenchmarkMetadata metadata;

  @JsonProperty("results")
  public List<TestResult> results;

  public static void saveResults(
    final List<TestResult> results,
    final String outputPath,
    final int cpuCount,
    final double totalMemoryMB
  ) throws IOException {
    final Path outputFile = Paths.get(outputPath);
    Files.createDirectories(outputFile.getParent());

    final Report resultsData = new Report();

    final BenchmarkMetadata metadata = new BenchmarkMetadata();
    metadata.language = "java";
    metadata.timestamp =
      java.time.LocalDateTime
        .now()
        .format(
          java.time.format.DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss")
        );
    metadata.javaVersion = System.getProperty("java.version");
    metadata.cpuCount = cpuCount;
    metadata.totalMemoryGb = totalMemoryMB / 1024.0;
    metadata.totalTests = results.size();

    resultsData.metadata = metadata;
    resultsData.results = results;

    final ObjectMapper mapper = new ObjectMapper();
    mapper
      .writerWithDefaultPrettyPrinter()
      .writeValue(outputFile.toFile(), resultsData);

    System.out.println("Results saved to " + outputFile);
  }
}
