{
  description = "Rust project with Nix dev shell and task apps";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # rustToolchain = pkgs.rust-bin.stable.latest.default;
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        commonPackages = with pkgs; [
          rustToolchain
          pkg-config
          openssl
          eza
          fd
          sqlx-cli
          cargo-readme
          cargo-nextest
          docker
          docker-compose
        ];

        mkTask = name: text:
          let
            script = pkgs.writeShellApplication {
              inherit name;
              runtimeInputs = commonPackages;
              text = text;
            };
          in {
            type = "app";
            program = "${script}/bin/${name}";
          };
      in {
        devShells.default = pkgs.mkShell {
          packages = commonPackages;
          DATABASE_URL = "postgres://postgres:password@localhost/api";
          RUST_LOG = "api=INFO,sqlx=INFO,tower_http=INFO,axum::rejection=trace";

          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
        };

        apps.check = mkTask "check" ''
          set -euo pipefail
          cargo clippy --all-targets --all-features -- -D warnings -D clippy::unwrap_used
          cargo fmt --all -- --check
          cargo nextest run
        '';
      });
}
