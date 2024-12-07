{ lib, pkgs, ... }: {
	languages.rust = {
		enable = true;
		channel = "nightly";
	};
}
