# setrise
Small, infinitely running, application to adjust your colorschemes based on a configured daytime.

## Howto use

1. Clone the repository.
2. `cd setrise`
3. Run `cargo install --path .`
4. Copy the example `.setriserc` file to `~/`
5. Adjust the `.setriserc` file to your needs
6. Copy the example `setrise.service` systemd file to `~/.config/systemd/user/` and adjust it to your requirements.

## Resources for hot reloading

* [Blogpost about nvim, GTK or Kitty](https://felix-kling.de/blog/2021/linux-toggle-dark-mode.html)
* [ArchWiki Dark mode switching](https://wiki.archlinux.org/title/Dark_mode_switching)

### Alacritty

For alacritty you need to make sure that you have `live_config_reload: true` in your configuration file.
