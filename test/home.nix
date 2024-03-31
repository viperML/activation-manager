{pkgs, ...}: {
  # xdg.configPath."hosts".source = "/etc/hosts";
  # xdg.configPath."someDir".source = "/tmp";
  # systemd.user.services."test" = {
  #   wantedBy = ["default.target"];
  #   serviceConfig.ExecStart = "${pkgs.coreutils}/bin/tail -f /etc/hosts";
  #   # serviceConfig.ExecStart = "${pkgs.coreutils}/bin/tail -f /etc/passwd";
  # };
  path."file".source = "/etc/hosts";
}
