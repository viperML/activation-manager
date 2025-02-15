{
  systemd.user.services."activation-manager" = {
    wantedBy = [ "default.target" ];
    script = ''
      activate="/etc/profiles/per-user/$USER/bin/activate"
      if [[ -f "$activate" ]]; then
        exec "$activate"
      else
        echo ":: Activation-manager not installed for this user"
      fi
    '';
  };
}
