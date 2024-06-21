{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays =[ ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      rec {

        # Development shell.
        # Run "nix develop" to activate.
        devShell = pkgs.mkShell {
          name = "dev";
          src = self;
          packages = with pkgs; [
            pkg-config
            openssl

            # LLVM and related dependencies
            cmake
            llvmPackages_15.libllvm
            llvmPackages_15.llvm
            libxml2
            libffi
            glibc
            libclang

            # Rust tooling

            # Snapshot testing
            # https://github.com/mitsuhiko/insta
            cargo-insta
            # Test runner
            # https://github.com/nextest-rs/nextest
            cargo-nextest
            # Rust dependency vulnerability checker
            # https://github.com/EmbarkStudios/cargo-deny
            cargo-deny

            # Webassembly tooling

            # "Official" WASM CLI tools
            # (wasm2wat, wat2wasm, wasm-objdump, ...)
            # https://github.com/WebAssembly/wabt
            wabt
            # Provides `wasm-opt` (WASM optimizer) and some other tools
            # https://github.com/WebAssembly/binaryen
            binaryen
            # Various WASM debugging and conversion tools
            # (partial overlap with "wabt")
            # https://github.com/bytecodealliance/wasm-tools
            wasm-tools
          ];

          env.LLVM_SYS_150_PREFIX = pkgs.llvmPackages_15.llvm.dev;
        };
      }
    );
}