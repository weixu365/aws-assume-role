#!/usr/bin/env bash

mkdir -p target
mv aws-assume-role-x86_64-apple-darwin/aws-assume-role target/aws-assume-role_darwin_amd64
mv aws-assume-role-aarch64-apple-darwin/aws-assume-role target/aws-assume-role_darwin_arm64
mv aws-assume-role-x86_64-pc-windows-gnu.exe/aws-assume-role.exe target/aws-assume-role_windows_amd64.exe
mv aws-assume-role-x86_64-unknown-linux-gnu/aws-assume-role target/aws-assume-role_linux_amd64
