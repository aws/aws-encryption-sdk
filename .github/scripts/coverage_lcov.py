#!/usr/bin/env python3
# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
"""Write a library's unit test line coverage as lcov for the test server dashboard.

The dashboard merges this report, line by line, with the coverage the
cross-language test servers give the same commit, so every SF: path is made
relative to the repository root. Standard library only.

    coverage_lcov.py lcov IN OUT [--base DIR]               lcov with other paths
    coverage_lcov.py cobertura XML OUT                      dotnet-coverage Cobertura
    coverage_lcov.py go COVERDIR OUT --module-dir DIR...    go -cover counters (GOCOVERDIR)
    coverage_lcov.py jacoco XML OUT --source-root DIR...    JaCoCo XML report

Report options: --keep PREFIX keeps only files under a repository-relative
prefix; --drop PREFIX removes them; --release-copy FROM=TO names a file under
FROM by its copy under TO when the two files are identical, for test servers
that run a committed copy of the generated code.
A missing or empty input writes no report and exits 0.
"""

import argparse
import filecmp
import glob
import os
import subprocess
import sys
import xml.etree.ElementTree as ElementTree

ROOT = os.path.realpath(os.path.join(os.path.dirname(__file__), "..", ".."))


def lcov_counts(path, base):
    """{path: {line: hits}} from an lcov file; relative paths are under base."""
    counts, hits = {}, None
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line.startswith("SF:"):
                hits = counts.setdefault(os.path.join(base, line[3:]), {})
            elif line.startswith("DA:") and hits is not None:
                number, count = line[3:].split(",")[:2]
                hits[int(number)] = max(hits.get(int(number), 0), int(count))
    return counts


def cobertura_counts(path):
    """A line listed by several classes (a method's compiler-generated
    classes) keeps the highest count."""
    root = ElementTree.parse(path).getroot()
    bases = [source.text for source in root.iter("source") if source.text]
    counts = {}
    for cls in root.iter("class"):
        name = cls.get("filename")
        if not os.path.isabs(name):
            name = next((os.path.join(base, name) for base in bases
                         if os.path.exists(os.path.join(base, name))), name)
        hits = counts.setdefault(name, {})
        for line in cls.iter("line"):
            number = int(line.get("number"))
            hits[number] = max(hits.get(number, 0), int(line.get("hits")))
    return counts


def go_counts(coverdir, module_dirs):
    """A Go text profile names each file by its import path, mapped to a file
    through the module directories' go.mod files. A block counts on every line
    it spans except a last line it only reaches the indentation of (a closing
    brace); a line in several blocks keeps the highest count."""
    if not glob.glob(os.path.join(coverdir, "covcounters.*")):
        return {}
    profile = os.path.normpath(coverdir) + ".txt"
    subprocess.run(["go", "tool", "covdata", "textfmt", "-i=" + coverdir, "-o", profile], check=True)
    modules = {}
    for directory in module_dirs:
        with open(os.path.join(directory, "go.mod")) as f:
            name = next(line.split()[1] for line in f if line.startswith("module "))
        modules[name] = os.path.abspath(directory)
    by_length = sorted(modules, key=len, reverse=True)
    counts, sources = {}, {}
    with open(profile) as f:
        for record in f.read().splitlines():
            if not record or record.startswith("mode:"):
                continue
            location, _, count = record.rsplit(" ", 2)
            name, span = location.rsplit(":", 1)
            module = next((m for m in by_length if name.startswith(m + "/")), None)
            if module is None:
                continue
            path = modules[module] + name[len(module):]
            start, end = span.split(",")
            start_line = int(start.split(".")[0])
            end_line, end_column = (int(part) for part in end.split("."))
            if end_line > start_line:
                if path not in sources:
                    try:
                        with open(path) as source:
                            sources[path] = source.read().splitlines()
                    except OSError:
                        sources[path] = []
                lines = sources[path]
                if end_line <= len(lines) and not lines[end_line - 1][:end_column - 1].strip():
                    end_line -= 1
            hits = counts.setdefault(path, {})
            for number in range(start_line, end_line + 1):
                hits[number] = max(hits.get(number, 0), int(count))
    return counts


def jacoco_counts(path, source_roots):
    """A line counts as hit when JaCoCo covered one of its instructions;
    JaCoCo keeps no hit counts."""
    counts = {}
    for package in ElementTree.parse(path).getroot().iter("package"):
        for sourcefile in package.findall("sourcefile"):
            relative = os.path.join(package.get("name"), sourcefile.get("name"))
            name = next((os.path.join(root, relative) for root in source_roots
                         if os.path.isfile(os.path.join(root, relative))), None)
            if name is None:
                continue
            hits = counts.setdefault(name, {})
            for line in sourcefile.findall("line"):
                hits[int(line.get("nr"))] = 1 if int(line.get("ci")) > 0 else 0
    return counts


def write_report(counts, out, keep, drop, release_copies):
    records = []
    for path in sorted(counts):
        real = os.path.realpath(path)
        if not real.startswith(ROOT + os.sep) or not counts[path]:
            continue
        name = os.path.relpath(real, ROOT)
        if keep and not any(name.startswith(prefix) for prefix in keep):
            continue
        if any(name.startswith(prefix) for prefix in drop):
            continue
        for source, copy in release_copies:
            if name.startswith(source):
                candidate = copy + name[len(source):]
                if os.path.isfile(os.path.join(ROOT, candidate)) and filecmp.cmp(
                        real, os.path.join(ROOT, candidate), shallow=False):
                    name = candidate
                break
        records.append("SF:%s\n" % name)
        records.extend("DA:%d,%d\n" % item for item in sorted(counts[path].items()))
        records.append("end_of_record\n")
    if not records:
        print("::warning::no line coverage for %s" % out)
        return
    os.makedirs(os.path.dirname(os.path.abspath(out)), exist_ok=True)
    with open(out, "w") as f:
        f.writelines(records)
    print("%s: %d files" % (out, sum(record.startswith("SF:") for record in records)))


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("format", choices=["lcov", "cobertura", "go", "jacoco"])
    parser.add_argument("source")
    parser.add_argument("out")
    parser.add_argument("--base", default=os.getcwd())
    parser.add_argument("--module-dir", action="append", default=[])
    parser.add_argument("--source-root", action="append", default=[])
    parser.add_argument("--keep", action="append", default=[])
    parser.add_argument("--drop", action="append", default=[])
    parser.add_argument("--release-copy", action="append", default=[])
    args = parser.parse_args()
    source, out = args.source, args.out
    if not os.path.exists(source):
        print("::warning::%s does not exist; no line coverage report" % source)
        return
    if args.format == "lcov":
        counts = lcov_counts(source, args.base)
    elif args.format == "cobertura":
        counts = cobertura_counts(source)
    elif args.format == "go":
        counts = go_counts(source, args.module_dir)
    else:
        counts = jacoco_counts(source, args.source_root)
    release_copies = [tuple(pair.split("=", 1)) for pair in args.release_copy]
    write_report(counts, out, args.keep, args.drop, release_copies)


if __name__ == "__main__":
    sys.exit(main())
