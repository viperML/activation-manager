{pkgs, ...}: {
  nodes.foo = {
    action = ''
      debug("Hello");
    '';
  };

  nodes.bar.action = ''
    time::sleep(time::Duration::from_secs(3)).await;
    debug("Goodbye");
  '';
}
