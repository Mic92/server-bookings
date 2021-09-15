with import <nixpkgs> {};
let
  fenixRev = "e917dae2552d6d50797a718bffba34102af082bf";
  fenix = pkgs.callPackage (fetchTarball "https://github.com/nix-community/fenix/archive/${fenixRev}.tar.gz") { };
in
mkShell {
  nativeBuildInputs = [
    bashInteractive
    (fenix.combine [
      fenix.minimal.rustc
      fenix.minimal.cargo
      fenix.targets.wasm32-unknown-unknown.latest.rust-std
    ])
    wasm-bindgen-cli
    trunk
  ];
}
