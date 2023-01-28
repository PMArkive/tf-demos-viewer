{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "nixpkgs/release-22.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    rust-overlay,
  }:
    utils.lib.eachDefaultSystem (system: let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      naersk = naersk.lib."${system}";
      rust = pkgs.rust-bin.stable.latest.default.override {
        targets = [ "wasm32-unknown-unknown" ];
      };
    in rec {
      # `nix develop`
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [rust bacon cargo-edit cargo-outdated clippy cargo-audit hyperfine valgrind wasm-pack pkg-config nodejs];

        buildInputs = with pkgs; [openssl];
        OPENSSL_NO_VENDOR = 1;
      };
    });
}
