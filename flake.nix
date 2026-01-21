{
  description = "A Nix-flake-based Typst development environment";
  inputs.nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1"; # unstable Nixpkgs
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
            };
          }
        );
    in
    {
      devShells = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.mkShellNoCC {
            packages =
              with pkgs;
              [
                typst
                typstyle
                tinymist
                # for the simulator
                SDL2
                pkg-config
                libiconv

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
                ]
              }:$LIBRARY_PATH
              export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [ pkgs.SDL2 ]}:$LD_LIBRARY_PATH
              ${pkgs.lib.optionalString pkgs.stdenv.isDarwin ''
                export DYLD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [ pkgs.SDL2 ]}:$DYLD_LIBRARY_PATH
              ''}
            '';
          };
        }
      );
    };
}
