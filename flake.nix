{
  description = "Programa de linha de comando para navegar entre os desafios, corrigí-los e dar dicas";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-darwin";
        overlays = [ (import rust-overlay) ];
      };
    in
    {
      devShells.aarch64-darwin = rec {
        default = devit;
        devit = pkgs.mkShell {
          name = "devit";
          packages = with pkgs; [
            rust-bin.stable.latest.default
            cargo-nextest rustfmt
            # self.packages.aarch64-darwin.devit
          ];
          inputsFrom = [ self.packages.aarch64-darwin.devit ];
          shellHook = ''alias devit="./target/debug/devit"'';
        };
      };

      packages.aarch64-darwin = rec {
        default = devit;
        devit = pkgs.rustPlatform.buildRustPackage {
          pname = "devit";
          version = "v0.1.0";
          src = ./.;
          buildInputs = with pkgs; with pkgs.darwin.apple_sdk; [
            frameworks.CoreFoundation
            frameworks.CoreServices
            frameworks.SystemConfiguration
          ];
          cargoLock = { lockFile = ./Cargo.lock; };
          meta = with pkgs.lib; {
            homepage = htpps://github.com/zoedsoupe/devit;
            license = licenses.mit;
            maintainers = [ maintainers.zoedsoupe ];
          };
        };
      };
    };
}
