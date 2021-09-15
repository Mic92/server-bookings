with import <nixpkgs> {};
mkShell {
  nativeBuildInputs = [
    bashInteractive
    rustup
    wasm-bindgen-cli
    trunk
  ];
}
