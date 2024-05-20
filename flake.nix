{
  description = "Wallheaven sync flake";
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.follows = "rust-overlay/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    ...
  }: let
    pkgs = import nixpkgs {system = "x86_64-linux";};
  in {
    nixpkgs.overlays = [rust-overlay.overlays.default];
    packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
      pname = "wallheaven_sync";
      version = "0.0.1";
      src = ./.;
      cargoBuildFlags = "--release";

      cargoLock = {
        lockFile = ./Cargo.lock;
      };

      nativeBuildInputs = [pkgs.pkg-config];
      PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
    };
    overlays.default = final: prev: {inherit (self.packages.${prev.system}) default;};
  };
}
