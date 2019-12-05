with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    (rustChannels.stable.rust.override { extensions = [ "rust-src" ];
                                         targets = ["x86_64-unknown-linux-musl"];})

    (rustChannels.stable.cargo.override {})
    # Example Build-time Additional Dependencies
    pkgconfig
  ];
  buildInputs = [
    # Example Run-time Additional Dependencies
    openssl
    git
    automake
    libtool
    protobuf
  ];

  # Set Environment Variables
  RUST_BACKTRACE = "full";

  PROTOC = "${protobuf}/bin/protoc";
}
