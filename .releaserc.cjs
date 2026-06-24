// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
  First run `make setup_semantic_release` to install the required dependencies.

  Using this config semantic-release will search for the latest tag
  evaluate all commits after that tag
  generate release notes and a version bump.
  It will commit these changes, push these changes, and publish a new version tag.

  This file requires a `--branches` option to function.
  This is to facilitate point releases if needed.

  `npx semantic-release --branches main`
*/

// This project has several runtimes
// each one has files that need to be updated.
// We model all the files and the runtimes here in this structure
const Runtimes = {
  net: {
    "AwsEncryptionSDK/runtimes/net/ESDK.csproj": {
      dependencies: [],
      assemblyInfo: [],
    },
  },
};

/**
 * @type {import('semantic-release').GlobalConfig}
 */
module.exports = {
  branches: ["mainline"],
  repositoryUrl: "git@github.com:aws/aws-encryption-sdk-dafny.git",
  plugins: [
    // Check the commits since the last release
    [
      "@semantic-release/commit-analyzer",
      {
        preset: "conventionalcommits",
        parserOpts: {
          noteKeywords: ["BREAKING CHANGE", "BREAKING CHANGES"],
        },
        presetConfig: {
          types: [
            { type: "feat", section: "Features" },
            { type: "fix", section: "Fixes" },
            { type: "chore", section: "Maintenance" },
            { type: "docs", section: "Maintenance" },
            { type: "revert", section: "Fixes" },
            { type: "style", section: "Miscellaneous" },
            { type: "refactor", section: "Miscellaneous" },
            { type: "perf", section: "Miscellaneous" },
            { type: "test", section: "Miscellaneous" },
          ],
        },
        releaseRules: [
          { type: "docs", release: "patch" },
          { type: "revert", release: "patch" },
          { type: "chore", release: "patch" },
        ],
      },
    ],
    // Based on the commits generate release notes
    [
      "@semantic-release/release-notes-generator",
      {
        preset: "conventionalcommits",
        presetConfig: {
          types: [
            {
              type: "feat",
              scope: "dafny",
              section: "Features -- All Languages",
              hidden: false,
            },
            {
              type: "feat",
              scope: "dotnet",
              section: "Features -- DotNet",
              hidden: false,
            },
            {
              type: "feat",
              scope: "go",
              section: "Features -- Go",
              hidden: false,
            },
            {
              type: "feat",
              scope: "rust",
              section: "Features -- Rust",
              hidden: false,
            },

            {
              type: "fix",
              scope: "dafny",
              section: "Fixes -- All Languages",
              hidden: false,
            },
            {
              type: "fix",
              scope: "dotnet",
              section: "Fixes -- DotNet",
              hidden: false,
            },
            { type: "fix", scope: "go", section: "Fixes -- Go", hidden: false },
            {
              type: "fix",
              scope: "rust",
              section: "Fixes -- Rust",
              hidden: false,
            },

            {
              type: "chore",
              scope: "dafny",
              section: "Maintenance -- All Languages",
              hidden: false,
            },
            {
              type: "chore",
              scope: "dotnet",
              section: "Maintenance -- DotNet",
              hidden: false,
            },
            {
              type: "chore",
              scope: "go",
              section: "Maintenance -- Go",
              hidden: false,
            },
            {
              type: "chore",
              scope: "rust",
              section: "Maintenance -- Rust",
              hidden: false,
            },
            {
              type: "chore",
              section: "Miscellaneous",
              hidden: false,
            },
            {
              type: "docs",
              scope: "dafny",
              section: "Maintenance -- All Languages",
              hidden: false,
            },
            {
              type: "docs",
              scope: "dotnet",
              section: "Maintenance -- DotNet",
              hidden: false,
            },
            {
              type: "docs",
              scope: "go",
              section: "Maintenance -- Go",
              hidden: false,
            },
            {
              type: "docs",
              scope: "rust",
              section: "Maintenance -- Rust",
              hidden: false,
            },
            {
              type: "revert",
              scope: "dafny",
              section: "Fixes -- All Languages",
              hidden: false,
            },
            {
              type: "revert",
              scope: "dotnet",
              section: "Fixes -- DotNet",
              hidden: false,
            },
            {
              type: "revert",
              scope: "go",
              section: "Fixes -- Go",
              hidden: false,
            },
            {
              type: "revert",
              scope: "rust",
              section: "Fixes -- Rust",
              hidden: false,
            },
            { type: "style", section: "Miscellaneous", hidden: false },
            { type: "refactor", section: "Miscellaneous", hidden: false },
            { type: "perf", section: "Miscellaneous", hidden: false },
            { type: "test", section: "Miscellaneous", hidden: false },
          ],
        },
      },
    ],
    // Update the change log with the generated release notes
    [
      "@semantic-release/changelog",
      {
        changelogFile: "CHANGELOG.md",
        changelogTitle: "# Changelog",
      },
    ],

    // Bump the various versions
    [
      "semantic-release-replace-plugin",
      {
        replacements: [
          // Update the version for all DotNet projects
          // Does not update the dependencies
          {
            files: Object.keys(Runtimes.net),
            from: "<Version>.*</Version>",
            to: "<Version>${nextRelease.version}</Version>",
            results: Object.keys(Runtimes.net).map(CheckResults),
            countMatches: true,
          },
        ],
      },
    ],
    // Commit and push changes the changelog and versions bumps
    [
      "@semantic-release/git",
      {
        assets: [
          "CHANGELOG.md",
          ...Object.values(Runtimes).flatMap((r) => Object.keys(r)),
          ...Object.values(Runtimes.net).flatMap((r) => r.assemblyInfo),
        ],
        message:
          "chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}",
      },
    ],
  ],
};

function CheckResults(file) {
  return {
    file,
    hasChanged: true,
    numMatches: 1,
    numReplacements: 1,
  };
}
