{
  	inputs = {
		utils.url = "github:numtide/flake-utils";
		naersk.url = "github:nmattia/naersk";
		fenix.url = "github:nix-community/fenix";
  	};

  	outputs = { self, nixpkgs, utils, naersk, fenix }:
	utils.lib.eachDefaultSystem (system: let
		pkgs = nixpkgs.legacyPackages."${system}";
		# Specify Rust Toolchain
		# Use Stable (Default)
		# naersk-lib = naersk.lib."${system}";
		# Use Nightly (provided by fenix)
		naersk-lib = naersk.lib."${system}".override {
			# Use Fenix to get nightly rust
			inherit (fenix.packages.${system}.minimal) cargo rustc;
		};
	in rec {
		# `nix build`
		packages.ray_game = naersk-lib.buildPackage {
			pname = "ray_game";
			root = ./.;
			buildInputs = with pkgs; [
				# Package location
				pkgconfig
				# Window and Input
				x11
				xorg.libXcursor
				xorg.libXrandr
				xorg.libXi

				# Vulkan
				vulkan-tools
				vulkan-headers
				vulkan-loader
				vulkan-validation-layers
				alsaLib # Sound support
				libudev # device management
				# lld # fast linker
			];
		};
		defaultPackage = packages.ray_game;

		# `nix run`
		apps.ray_game = utils.lib.mkApp {
			drv = packages.ray_game;
		};
		defaultApp = apps.ray_game;

		# `nix develop`
		devShell = pkgs.mkShell {
			buildInputs = packages.ray_game.buildInputs ++ [ pkgs.llvmPackages_12.lldClang.bintools ];
			LD_LIBRARY_PATH = "${nixpkgs.lib.makeLibraryPath packages.ray_game.buildInputs}";
  			hardeningDisable = [ "fortify" ];
  			NIX_CFLAGS_LINK = "-fuse-ld=lld";
		};
	});
}