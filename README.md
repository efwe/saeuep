# StandAlone Easy Ubuntu Emoji Picker - sÄÜp

* Find the ubuntu emoji picker library
* learn how to link against it in rust
* write the most simple program to use this lib
* invoke - select - return the emoji
* Prophecy: The returning thing will be the problem because nobody every agreed on what an emoji actually is...
* create a logo with sÄÜp (the coolest project name on earth)

## Buildah / Podman

The plan is to have a `bui.sh` which uses the `Buildah` tool to 
prepare a `Ubuntu:22.04` image with the addtional features:

* gtk4 + build essentials
* rust / crates
* the actual source

Then we prepare a run-comman with mount-points so that the executable
is delivered to the host directly

Note: For development we may try
`podman run -it --name saup-dev -v /home/fw/Devel/saeuep:/saeuep saeuep:latest  /bin/bash`

### Details

* https://www.redhat.com/sysadmin/buildah-unshare-command


## GTK - Hell Yeah - 28 Years Later

* https://gtk-rs.org/
* https://docs.gtk.org/gtk4/class.EmojiChooser.html


## Prior Art

Of course everybody and their cat already invented something like this. Here is what I found during my investigations:

* [Emote - Python](https://github.com/tom-james-watson/Emote) 
* [im-emoji - C++](https://github.com/GaZaTu/im-emoji-picker)


## Wez integration

* learn how to invoke that tool
* learn how to use the result to display it in Wez term


