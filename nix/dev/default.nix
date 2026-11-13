{ inputs, ... }: {
	imports = [
		inputs.git_hook.flakeModule
		inputs.rust_flake.flakeModules.default
		inputs.rust_flake.flakeModules.nixpkgs
	];

	perSystem = { config, pkgs, inputs', ... }: {
		devShells.default = pkgs.mkShell {
			RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
			RUST_BACKTRACE = "1";
			RUSTFLAGS = "-A warnings";

			inputsFrom = [
				config.pre-commit.devShell
				config.devShells.rust
			];

			nativeBuildInputs = [
				pkgs.license-generator
				pkgs.bash-language-server
				pkgs.mdwatch
				pkgs.nixd
				pkgs.nixpkgs-fmt
				pkgs.git
				pkgs.git-lfs
				pkgs.cocogitto
				pkgs.pre-commit
				pkgs.pkg-config
				pkgs.nushell
				pkgs.cargo-info
				pkgs.cargo-whatfeatures
				pkgs.cargo-readme
			];

			buildInputs = [
				pkgs.openssl
				pkgs.glib
			];

			shellHook = ''
				${pkgs.git-lfs}/bin/git-lfs install --local --skip-repo --quiet > /dev/null 2>&1

				${config.pre-commit.installationScript} > /dev/null 2>&1
			'';
		};

		rust-project.toolchain = inputs'.fenix.packages.stable.toolchain;

		pre-commit.check.enable = true;

		pre-commit.settings.hooks.check-symlinks.enable = true;
		pre-commit.settings.hooks.check-merge-conflicts.enable = true;
		pre-commit.settings.hooks.check-added-large-files.enable = true;
		pre-commit.settings.hooks.check-added-large-files.args = [
			"--maxkb=50"
			"--enforce-all"
		];

		pre-commit.settings.hooks.cog.enable = true;
		pre-commit.settings.hooks.cog.entry = "${pkgs.cocogitto}/bin/cog verify --file";
		pre-commit.settings.hooks.cog.stages = [
			"commit-msg"
		];

		pre-commit.settings.hooks.ripsecrets.enable = true;

		pre-commit.settings.hooks.check-yaml.enable = true;
		pre-commit.settings.hooks.check-json.enable = true;
		pre-commit.settings.hooks.check-toml.enable = true;

		pre-commit.settings.hooks.trim-trailing-whitespace.enable = true;

		pre-commit.settings.hooks.end-of-file-fixer.enable = true;
		pre-commit.settings.hooks.end-of-file-fixer.excludes = [
			"^target/"
			"^build/"
			"^node_modules/"
			"^vendor/"
			"^\\.git/"
			"\\.min\\.js$"
			"\\.min\\.css$"
			"\\.svg$"
			"\\.png$"
			"\\.jpg$"
			"\\.ico$"
		];

		pre-commit.settings.hooks = {
			cargo_clippy.enable = true;
			cargo_clippy.name = "cargo_clippy";
			cargo_clippy.description = "Run cargo clippy on commit";
			cargo_clippy.entry = "${pkgs.nushell}/bin/nu -c '
				with-env [PATH ($env.PATH | prepend \"${inputs'.fenix.packages.stable.toolchain}/bin\")] {
					cargo clippy \
						--all-targets \
						--all-features \
						-- \
						-D warnings
				}
			'";
			cargo_clippy.files = "\\.rs$";
			cargo_clippy.pass_filenames = false;

			cargo_test.enable = true;
			cargo_test.name = "cargo_test";
			cargo_test.description = "Run cargo test on commit";
			cargo_test.entry = "${pkgs.nushell}/bin/nu -c '
				with-env [PATH ($env.PATH | prepend \"${inputs'.fenix.packages.stable.toolchain}/bin\")] {
					cargo test --all-targets --all-features
				}
			'";
			cargo_test.files = "\\.rs$";
			cargo_test.pass_filenames = false;
		};

		pre-commit.settings.hooks.cargo-readme.enable = true;
		pre-commit.settings.hooks.cargo-readme.name = "cargo-readme";
		pre-commit.settings.hooks.cargo-readme.description = "Automatically update docs based on rust comment";
		pre-commit.settings.hooks.cargo-readme.entry = "${pkgs.nushell}/bin/nu -c '
			${pkgs.cargo-readme}/bin/cargo-readme readme | save README.md --force
		'";
		pre-commit.settings.hooks.cargo-readme.files = "^(src/.*\\.rs|Cargo\\.toml)$";
		pre-commit.settings.hooks.cargo-readme.pass_filenames = false;
	};
}
