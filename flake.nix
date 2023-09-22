{
  description = "SuckIT devshell";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-23.05";
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
      in
      with pkgs;
      {
        devShell = mkShell {
          buildInputs = [
            rust-bin.stable.latest.default
            openssl
            pkg-config
          ];
        };
      }
    );
}
