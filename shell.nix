{ pkgs ? import <nixpkgs> { } }:
with pkgs;

let
  buildInputs = [ cmake dbus gcc-unwrapped libgit2 openssl pkgconfig zlib rust-analyzer ];

in
pkgs.mkShell {
  inherit buildInputs;

  shellHook = "export CFG_DISABLE_CROSS_TESTS=1";

  LD_LIBRARY_PATH =
    "${lib.strings.makeLibraryPath buildInputs}";
}
