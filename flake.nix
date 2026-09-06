{
  description = "A fast, lightweight, native Spotify client built with Rust and egui";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # rust-toolchain.toml pins the compiler so local builds and CI agree.
    # This reads that file rather than restating the version here.
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      systems = [
        "aarch64-darwin"
        "x86_64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          f (
            import nixpkgs {
              inherit system;
              overlays = [ (import rust-overlay) ];
            }
          )
        );
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages =
            with pkgs;
            [
              (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
              rust-analyzer
              pkg-config
              just
              # libprojectM (MilkDrop) is built from source by CMake, and its
              # bindings by bindgen, which needs libclang.
              cmake
              rustPlatform.bindgenHook
              # projectm-eval probes for these; the shell's DEVELOPER_DIR
              # points at the nix SDK, which breaks the /usr/bin xcrun shims.
              bison
              flex
            ]
            ++ lib.optionals stdenv.hostPlatform.isDarwin [
              apple-sdk
            ]
            ++ lib.optionals stdenv.hostPlatform.isLinux [
              alsa-lib
              libpulseaudio
              libxkbcommon
              wayland
              libGL
              libx11
              libxcursor
              libxi
              libxrandr
            ];
          # The GUI dlopens its Wayland, X11 and GL libraries at run time.
          LD_LIBRARY_PATH = pkgs.lib.optionalString pkgs.stdenv.hostPlatform.isLinux (
            pkgs.lib.makeLibraryPath (
              with pkgs;
              [
                libxkbcommon
                wayland
                libGL
                libx11
                libxcursor
                libxi
                libxrandr
              ]
            )
          );
        };
      });

      packages = forAllSystems (
        pkgs:
        let
          snoop =
            let
              toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
              rustPlatform = pkgs.makeRustPlatform {
                cargo = toolchain;
                rustc = toolchain;
              };
              cmakeWithLibdir = pkgs.writeShellScript "cmake-snoop" ''
                if [[ "$1" == "--build" ]]; then
                  exec ${pkgs.cmake}/bin/cmake "$@"
                else
                  exec ${pkgs.cmake}/bin/cmake "$@" -DCMAKE_INSTALL_LIBDIR=lib
                fi
              '';
              runtimeLibs = pkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux (
                with pkgs;
                [
                  libxkbcommon
                  wayland
                  libGL
                  libx11
                  libxcursor
                  libxi
                  libxrandr
                ]
              );
            in
            rustPlatform.buildRustPackage {
              pname = "snoop";
              version = (pkgs.lib.importTOML ./Cargo.toml).package.version;
              src = self;

              # The lock file contains git dependencies. fetchCargoVendor includes
              # them in the fixed-output dependency tree, unlike cargoLock alone.
              cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
                pname = "snoop";
                version = (pkgs.lib.importTOML ./Cargo.toml).package.version;
                src = self;
                hash = "sha256-tOTVA5TR/1uZwnHMWJ4OAcyEive0EM3h8titY8WuOkc=";
              };

              nativeBuildInputs =
                with pkgs;
                [
                  pkg-config
                  # libprojectM (MilkDrop) is built from source by CMake, and
                  # its bindings by bindgen, which needs libclang.
                  cmake
                  rustPlatform.bindgenHook
                ]
                ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux [ makeWrapper ];
              buildInputs =
                pkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux (
                  with pkgs;
                  [
                    alsa-lib
                    libpulseaudio
                    # libprojectM links OpenGL directly and its GL loader needs
                    # X11 headers while it is built.
                    libGL
                    libx11
                  ]
                )
                ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isDarwin [ pkgs.apple-sdk ];

              # projectm-sys expects CMake to install into lib/, while CMake
              # defaults to lib64/ on NixOS.
              env.CMAKE = "${cmakeWithLibdir}";

              # The GUI dlopens its Wayland, X11 and GL libraries at run time.
              postFixup = pkgs.lib.optionalString pkgs.stdenv.hostPlatform.isLinux ''
                wrapProgram $out/bin/snoop \
                  --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath runtimeLibs}
              '';

              postInstall = pkgs.lib.optionalString pkgs.stdenv.hostPlatform.isLinux ''
                install -Dm644 packaging/applications/snoop.desktop \
                  $out/share/applications/snoop.desktop
                install -Dm644 packaging/icons/snoop.svg \
                  $out/share/icons/hicolor/scalable/apps/snoop.svg
              '';

              meta = {
                description = "Fast native Spotify client with local playback and Spotify Connect";
                homepage = "https://github.com/dappermint/snoop";
                license = pkgs.lib.licenses.mit;
                mainProgram = "snoop";
              };
            };

          snoop-app =
            let
              version = pkgs.lib.getVersion snoop;
              build = pkgs.lib.head (pkgs.lib.splitString "-" version);
              icon =
                pkgs.runCommand "snoop-icon"
                  {
                    nativeBuildInputs = [ pkgs.icnsify ];
                  }
                  ''
                    icnsify ${./packaging/macos/icon-1024.png} -o $out
                  '';
            in
            pkgs.runCommand "snoop-app"
              {
                meta = {
                  description = "Snoop as a macOS app bundle";
                  homepage = "https://github.com/dappermint/snoop";
                  license = pkgs.lib.licenses.mit;
                  platforms = pkgs.lib.platforms.darwin;
                };
              }
              ''
                app="$out/Applications/Snoop.app/Contents"
                mkdir -p "$app/MacOS" "$app/Resources"
                cp ${snoop}/bin/snoop "$app/MacOS/snoop"
                chmod 755 "$app/MacOS/snoop"
                cp ${icon} "$app/Resources/snoop.icns"
                sed -e "s/__VERSION__/${version}/g" -e "s/__BUILD__/${build}/g" \
                  ${./packaging/macos/Info.plist} > "$app/Info.plist"
                /usr/bin/codesign --force --sign - \
                  "$out/Applications/Snoop.app"
                /usr/bin/codesign --verify --strict \
                  "$out/Applications/Snoop.app"
              '';
        in
        {
          default = snoop;
          inherit snoop;
        }
        // pkgs.lib.optionalAttrs pkgs.stdenv.hostPlatform.isDarwin {
          inherit snoop-app;
        }
      );

      formatter = forAllSystems (pkgs: pkgs.nixfmt-tree);
    };
}
