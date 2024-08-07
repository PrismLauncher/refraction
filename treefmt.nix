{
  projectRootFile = ".git/config";

  programs = {
    actionlint.enable = true;
    deadnix.enable = true;
    nixfmt.enable = true;
    prettier.enable = true;
    rustfmt.enable = true;
    statix.enable = true;
  };
}
