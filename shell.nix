let
  mozillaOverlay =
    import (builtins.fetchGit {
      url = "https://github.com/mozilla/nixpkgs-mozilla.git";
      rev = "57c8084c7ef41366993909c20491e359bbb90f54";
    });
  nixpkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
  rust-stable = with nixpkgs; ((rustChannelOf { date = "2021-05-06"; channel = "stable"; }).rust.override {
    targets = [ "wasm32-unknown-unknown" ];
  });
  updateContract = nixpkgs.writeShellScriptBin "update-contract" ''
    update-single-contract erc20
  '';
  updateSingleContract = nixpkgs.writeShellScriptBin "update-single-contract" ''
    rm -f resources/test/$1_contract.wasm
    cd examples/$1-contract
    cargo build --release --target=wasm32-unknown-unknown
    cd ../../
    mv examples/$1-contract/target/wasm32-unknown-unknown/release/$1_contract.wasm resources/test/$1_contract.wasm \
      & echo "==> update $1" \
      & echo "==> `ls -l resources/test/$1_contract.wasm`"
  '';
  testScript = nixpkgs.writeShellScriptBin "run-test" ''
    cd sewup
    cargo test -p sewup --no-default-features --features=$1 -- --nocapture | tee /tmp/vm_errors && exit $(grep ERROR /tmp/vm_errors | wc -l)
    cd ../
  '';
  exampleTestScript = nixpkgs.writeShellScriptBin "run-example-test" ''
    cd examples/$1-contract
    cargo test
    cd ../../
  '';
in
with nixpkgs; pkgs.mkShell {
  buildInputs = [
    clang
    cmake
    pkg-config
    rust-stable
    boost
    updateContract
    updateSingleContract
    testScript
    exampleTestScript
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
  ];

  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
  PROTOC = "${protobuf}/bin/protoc";
  ROCKSDB_LIB_DIR = "${rocksdb}/lib";
}