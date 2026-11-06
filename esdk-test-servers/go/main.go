// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Runnable entry point for the Go Language_Server: binds an rpcv2Cbor HTTP
// endpoint on a port taken from (in order) the first CLI argument, the
// ESDK_TESTSERVER_PORT env var, or the default 8099.

package main

import (
	"context"
	"errors"
	"fmt"
	"net"
	"net/http"
	"os"
	"os/signal"
	"strconv"
	"syscall"
	"time"
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

	// Return from main on SIGTERM: a binary built with -cover writes its
	// coverage counters only on a normal exit.
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()

	addr := fmt.Sprintf("127.0.0.1:%d", port)
	listener, err := net.Listen("tcp", addr)
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to bind %s: %v\n", addr, err)
		os.Exit(1)
	}
	fmt.Fprintf(os.Stderr, "listening at http://%s\n", addr)
	server := &http.Server{Handler: newHandler(state)}
	go func() {
		if err := server.Serve(listener); !errors.Is(err, http.ErrServerClosed) {
			fmt.Fprintf(os.Stderr, "server error: %v\n", err)
			os.Exit(1)
		}
	}()

	<-ctx.Done()
	shutdown, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	server.Shutdown(shutdown)
}
