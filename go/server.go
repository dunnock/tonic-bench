/*
 *
 * Copyright 2015 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

//go:generate protoc -I ../helloworld --go_out=plugins=grpc:../helloworld ../helloworld/helloworld.proto

// Package main implements a server for Greeter service.
package main

import (
	"context"
	"log"
	"net"

	pb "github.com/dunnock/tonic-bench/proto"

	"google.golang.org/grpc"
)

const (
	port = ":50050"
)

// server is used to implement hellobench.GreeterServer.
type server struct {
	pb.UnimplementedGreeterServer
}

// SayEmpty implements hellobench.GreeterServer
func (s *server) SayEmpty(ctx context.Context, in *pb.Empty) (*pb.Empty, error) {
	return &pb.Empty{}, nil
}

// SaySomething implements hellobench.GreeterServer
func (s *server) SaySomething(ctx context.Context, in *pb.Something) (*pb.Something, error) {
	return &pb.Something{Text: "some reply string"}, nil
}

func main() {
	lis, err := net.Listen("tcp", port)
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	s := grpc.NewServer()
	pb.RegisterGreeterServer(s, &server{})
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
