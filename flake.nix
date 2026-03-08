{
  description = "A Nix-flake-based Typst development environment";
  inputs.nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1"; # unstable Nixpkgs
  inputs.fenix = {
    url = "https://flakehub.com/f/nix-community/fenix/0.1";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs =
    { self, ... }@inputs:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        inputs.nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [
                inputs.self.overlays.default
              ];
            };
          }
        );
    in
    {

      overlays.default = final: prev: {
        rustToolchain =
          with inputs.fenix.packages.${prev.stdenv.hostPlatform.system};
          combine (
            with stable;
            [
              clippy
              rustc
              cargo
              rustfmt
              rust-src

              targets.riscv32imc-unknown-none-elf.stable.rust-std
            ]
          );

      };

      devShells = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.mkShellNoCC {
            packages =
              with pkgs;
              [
                rustToolchain
                typst
                typstyle
                tinymist
                # for the simulator
                SDL2
                pkg-config
                libiconv
                vips

                esp-generate
                espflash
                espup

              ]
              ++ (with typstPackages; [
                # Typst packages
              ]);
            shellHook = ''
              export LIBRARY_PATH=${
                pkgs.lib.makeLibraryPath [
                  pkgs.SDL2
                  pkgs.libiconv
                  pkgs.vips
                ]
              }:$LIBRARY_PATH
              export LD_LIBRARY_PATH=${
                pkgs.lib.makeLibraryPath [
                  pkgs.SDL2
                  pkgs.vips
                ]
              }:$LD_LIBRARY_PATH
              ${pkgs.lib.optionalString pkgs.stdenv.isDarwin ''
                export DYLD_LIBRARY_PATH=${
                  pkgs.lib.makeLibraryPath [
                    pkgs.SDL2
                    pkgs.vips
                  ]
                }:$DYLD_LIBRARY_PATH
              ''}
            '';
          };
        }
      );
    };
}
