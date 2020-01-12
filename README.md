# applinux

Applinux is a cli tool to pack a linux desktop app from binaries and Appimages. 
If you are too lazy to create a .desktop and move it to its place you will appreciate this tool.
Applinux creates a copy of your binary and you icon, pack them together in a single folder and moves them to $HOME/.local/share.

## Usage

Applinux contains few commands, the main one is `new`. In this case way you can specify the binary file and the icon and create the app:

> $ applinux new <appname> --rm --bin <path/to/appname> --icon <path/to/icon>

The flag `--rm` specifies to remove source binaries and icons. Since applinuxs pack together apps and icons and move them to `$HOME/.local/share` in order to update them you can directly update them in share directory or use `update` command to make applinux make it for you:

> $ applinux update <appname> --bin <path/to/new/appname> --icon <and/or/path/to/new/icon>

You can actually specify packages destination if you don't like `$HOME/.local/share`. Remember: you will need root permissions to create and move files in root owned folders:

> $ sudo applinux new <appname> --bin <path/to/appname> --dest <path/to/root/destination>

## Config

If you are lazy and sqeamish too, specify an alternative destination path for each of your binaries is too much. In this case you can update the config.yaml file you can find nowhere. Not my business sorry.
