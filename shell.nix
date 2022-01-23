{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell rec {
	buildInputs = with pkgs; [
		llvmPackages_latest.llvm
		llvmPackages_latest.clang
		llvmPackages_latest.bintools
		zlib.out
		rustup
		llvmPackages_latest.lld
		mold
		python3
		git

		rust-analyzer
		nodejs
		neovim
		nodePackages.coc-rust-analyzer
		vimPlugins.coc-rust-analyzer
	];
	RUSTC_VERSION = "stable";
	# https://github.com/rust-lang/rust-bindgen#environment-variables
	LIBCLANG_PATH= pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
	HISTFILE=toString ./.history;
	shellHook = ''
		# rm -rf ~/.config/coc/extensions/coc-rust-analyzer-data/rust-analyzer
		# ln -s $(eval which rust-analyzer) ~/.config/coc/extensions/coc-rust-analyzer-data/rust-analyzer
		export PATH=$PATH:~/.cargo/bin
		export PATH=$PATH:~/.rustup/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
		export RUSTFLAGS="-C linker=${pkgs.llvmPackages_latest.clang}/bin/clang -C link-arg=-fuse-ld=${pkgs.mold}/bin/mold -Ctarget-cpu=native"
		PS1="\e[32;1mnix-shell: \e[34m\w \[\033[00m\]\n↳ "
		'';
	# Add libvmi precompiled library to rustc search path
	RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
		pkgs.libvmi
	]);
	# Add libvmi, glibc, clang, glib headers to bindgen search path
	BINDGEN_EXTRA_CLANG_ARGS = 
	# Includes with normal include path
	(builtins.map (a: ''-I"${a}/include"'') [
		pkgs.libvmi
		pkgs.glibc.dev 
	])
	# Includes with special directory paths
	++ [
		''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
		''-I"${pkgs.glib.dev}/include/glib-2.0"''
		''-I${pkgs.glib.out}/lib/glib-2.0/include/''
	];
}
