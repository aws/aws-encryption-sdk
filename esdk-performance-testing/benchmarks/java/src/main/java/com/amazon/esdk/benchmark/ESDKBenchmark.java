// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package com.amazon.esdk.benchmark;

import com.amazon.esdk.benchmark.model.Config;
import com.amazon.esdk.benchmark.model.TestResult;
import com.amazonaws.encryptionsdk.AwsCrypto;
import com.amazonaws.encryptionsdk.CryptoResult;
import java.nio.ByteBuffer;
import java.security.SecureRandom;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import javax.crypto.KeyGenerator;
import javax.crypto.SecretKey;
import me.tongfei.progressbar.ProgressBar;
import software.amazon.cryptography.materialproviders.IKeyring;
import software.amazon.cryptography.materialproviders.MaterialProviders;
import software.amazon.cryptography.materialproviders.model.AesWrappingAlg;
import software.amazon.cryptography.materialproviders.model.CreateRawAesKeyringInput;
import software.amazon.cryptography.materialproviders.model.MaterialProvidersConfig;

/**
 * ESDK Performance Benchmark Suite - Java Implementation
 *
 * <p>This class provides comprehensive performance testing for the AWS Encryption SDK (ESDK) Java
 * runtime, measuring throughput, latency, memory usage, and scalability.
 */
public final class ESDKBenchmark {

  final Config config;
  final AwsCrypto crypto;
  final IKeyring keyring;
  // System information
  final int cpuCount;
  final long totalMemoryMB;

  public ESDKBenchmark(final String configPath) throws Exception {
    this.config = Config.loadConfig(configPath);

    // System info
    this.cpuCount = Runtime.getRuntime().availableProcessors();
    this.totalMemoryMB = Runtime.getRuntime().maxMemory() / (1024 * 1024);

    // Initialize AWS Crypto
    this.crypto = AwsCrypto.standard();

    // Generate a 256-bit AES key for testing
    final KeyGenerator aesGen = KeyGenerator.getInstance("AES");
    aesGen.init(256, new SecureRandom());
    final SecretKey encryptionKey = aesGen.generateKey();
    final ByteBuffer keyBytes = ByteBuffer.wrap(encryptionKey.getEncoded());

    // Create Raw AES keyring using Material Providers
    final String keyNamespace = "esdk-performance-test";
    final String keyName = "test-aes-256-key";

    final CreateRawAesKeyringInput keyringInput = CreateRawAesKeyringInput
      .builder()
      .keyName(keyName)
      .keyNamespace(keyNamespace)
      .wrappingKey(keyBytes)
      .wrappingAlg(AesWrappingAlg.ALG_AES256_GCM_IV12_TAG16)
      .build();

    final MaterialProviders matProv = MaterialProviders
      .builder()
      .MaterialProvidersConfig(MaterialProvidersConfig.builder().build())
      .build();

    this.keyring = matProv.CreateRawAesKeyring(keyringInput);

    System.out.println(
      "Initialized ESDK Benchmark - CPU cores: " +
      cpuCount +
      ", Memory: " +
      (totalMemoryMB / 1024.0) +
      "GB"
    );
  }

  /** Run a single encrypt-decrypt cycle and measure performance */
  public EncryptDecryptResult runEncryptDecryptCycle(final byte[] data) {
    final var encryptionContext = Collections.singletonMap(
      "purpose",
      "performance-test"
    );

    // Measure encryption
    final long encryptStart = System.nanoTime();
    final CryptoResult<byte[], ?> encryptResult = crypto.encryptData(
      keyring,
      data,
      encryptionContext
    );
    final long encryptTime = System.nanoTime() - encryptStart;

    final byte[] ciphertext = encryptResult.getResult();

    // Measure decryption
    final long decryptStart = System.nanoTime();
    final CryptoResult<byte[], ?> decryptResult = crypto.decryptData(
      keyring,
      ciphertext
    );
    final long decryptTime = System.nanoTime() - decryptStart;

    final byte[] decryptedData = decryptResult.getResult();

    // Verify data integrity
    if (!Arrays.equals(data, decryptedData)) {
      throw new RuntimeException("Decrypted data does not match original data");
    }

    return new EncryptDecryptResult(
      encryptTime / 1_000_000.0, // Convert to milliseconds
      decryptTime / 1_000_000.0,
      ciphertext.length
    );
  }

  public List<TestResult> runAllBenchmarks() {
    System.out.println("Starting comprehensive ESDK benchmark suite");
    final List<TestResult> allResults = new ArrayList<>();

    // Get test parameters from config
    final List<Integer> dataSizes = new ArrayList<>();
    if (config.dataSizes.small != null) dataSizes.addAll(
      config.dataSizes.small
    );
    if (config.dataSizes.medium != null) dataSizes.addAll(
      config.dataSizes.medium
    );
    if (config.dataSizes.large != null) dataSizes.addAll(
      config.dataSizes.large
    );

    // Calculate actual total tests
    final int throughputTests = dataSizes.size();
    final int memoryTests = dataSizes.size();
    final int concurrentTests =
      dataSizes.size() *
      (int) config.concurrencyLevels.stream().filter(c -> c > 1).count();
    final int totalTests = throughputTests + memoryTests + concurrentTests;

    System.out.println("Running " + totalTests + " total tests");

    try (
      final ProgressBar pb = new ProgressBar("Running benchmarks", totalTests)
    ) {
      // Throughput tests
      for (final int dataSize : dataSizes) {
        try {
          final TestResult result = Tests.runThroughputTest(
            this,
            dataSize,
            config.iterations.measurement
          );
          if (result != null) {
            allResults.add(result);
            System.out.println(
              "Throughput test completed: " +
              String.format("%.2f", result.opsPerSecond) +
              " ops/sec"
            );
            System.out.flush();
            System.out.println(
              "Throughput test completed - Ops/sec: " +
              String.format("%.2f", result.opsPerSecond) +
              ", MB/sec: " +
              String.format("%.2f", result.bytesPerSecond / (1024 * 1024))
            );
          }
        } catch (final Exception e) {
          System.err.println(
            "Throughput test failed for data size " +
            dataSize +
            " bytes: " +
            e.getMessage()
          );
        }
        System.out.flush();
        pb.step();
        System.out.flush();
      }

      // Memory tests
      for (final int dataSize : dataSizes) {
        try {
          final TestResult result = Tests.runMemoryTest(this, dataSize);
          allResults.add(result);
          System.out.println(
            "Memory test completed: " +
            String.format("%.2f", result.peakMemoryMb) +
            " MB peak"
          );
          System.out.flush();
        } catch (final Exception e) {
          System.err.println(
            "Memory test failed for data size " +
            dataSize +
            " bytes: " +
            e.getMessage()
          );
        }
        System.out.flush();
        pb.step();
        System.out.flush();
      }

      // Concurrent tests
      for (final int dataSize : dataSizes) {
        for (final int concurrency : config.concurrencyLevels) {
          if (concurrency > 1) { // Skip single-threaded for concurrent tests
            try {
              final TestResult result = Tests.runConcurrentTest(
                this,
                dataSize,
                concurrency,
                5
              );
              allResults.add(result);
              System.out.println(
                "Concurrent test completed: " +
                String.format("%.2f", result.opsPerSecond) +
                " ops/sec @ " +
                concurrency +
                " threads"
              );
            } catch (final Exception e) {
              System.err.println(
                "Concurrent test failed for data size " +
                dataSize +
                " bytes with " +
                concurrency +
                " threads: " +
                e.getMessage()
              );
            }
            System.out.flush();
            pb.step();
            System.out.flush();
          }
        }
      }
    }

    System.out.println(
      "Benchmark suite completed. Total results: " + allResults.size()
    );
    return allResults;
  }

  public record EncryptDecryptResult(
    double encryptTimeMs,
    double decryptTimeMs,
    int ciphertextSize
  ) {}
}
