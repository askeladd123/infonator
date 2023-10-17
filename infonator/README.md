# infonator
A program that simply displays a set of system information.

I am making this program because I wanted to make a desktop environment without a status bar.
> I'd rather have a large window showing all the information I need when I hold down a key, than a small bar covering the screen all the time

**philosophy**
1. fast startup time
2. it is my job to display information, and the users job to aquire it through scripts
3. good defaults: despite giving responsibilty to the user, it is still important to work well out of the box

> note: point 2 is subject to change, as I may find good rust libraries for aquiring information, but this may increase the binary since, hence the startup time. 

## configuration
To configure the program and aquire system information, you will use the program from the command line to set internal variables. This means you can configure the program from anywhere, and instead of learning yet another config file syntax, you can make a script in any language you want, and configure from there. To aquire system information, you pass and executable file that outputs corresponding information.

This has advantages:
- you can use a familiar language to configure, with for example syntax highlighting
- takes the burden off developers to keep cross-platform compatibilty

## information
- primary
  - wifi
  - battery
  - time
  - volume
- secondary
  - ip address
  - date
  - brightness
  - cpu temperature
  - ram percentage
- window manager: open windows
