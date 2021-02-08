{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
	buildInputs = with pkgs; [
		vulkan-loader
		cmake
		python3
		shaderc

		# for X11 users
		xorg.libXcursor
		xorg.libXrandr
		xorg.libXi
	];
	LD_LIBRARY_PATH = "${pkgs.vulkan-loader}/lib:${pkgs.xorg.libXcursor}/lib:${pkgs.xorg.libXrandr}/lib:${pkgs.xorg.libXi}/lib";
}
