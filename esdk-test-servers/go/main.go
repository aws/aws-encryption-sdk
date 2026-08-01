// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Runnable entry point for the Go Language_Server: binds an rpcv2Cbor HTTP
// endpoint on a port taken from (in order) the first CLI argument, the
// ESDK_TESTSERVER_PORT env var, or the default 8099.

package main

import (
	"fmt"
	"net"
	"net/http"
	"os"
	"strconv"
)

func main() {
	port := 8099
	raw := os.Getenv("ESDK_TESTSERVER_PORT")
	if len(os.Args) > 1 {
		raw = os.Args[1]
	}
	if parsed, err := strconv.Atoi(raw); err == nil && parsed > 0 && parsed <= 65535 {
		port = parsed
	}

	state, err := newAppState()
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to construct the material providers client: %v\n", err)
		os.Exit(1)
	}

	addr := fmt.Sprintf("127.0.0.1:%d", port)
	listener, err := net.Listen("tcp", addr)
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to bind %s: %v\n", addr, err)
		os.Exit(1)
	}
	fmt.Fprintf(os.Stderr, "listening at http://%s\n", addr)
	server := &http.Server{Handler: newHandler(state)}
	if err := server.Serve(listener); err != nil {
		fmt.Fprintf(os.Stderr, "server error: %v\n", err)
		os.Exit(1)
	}
}
