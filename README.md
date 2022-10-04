# change-vol-pactl

Utility to change the volume of multiple sinks using pactl in one go.

If in a line of the `pactl list sinks short` output a string present in the `MATCHES` definition is found, then the corresponding sink is updated with the argument passed.

`config.h` can be edited to make the program search for whatever strings might be of interest in the `pactl list sinks short` output.

## Usage
`change-pactl-volume {volume specification | toggle}`

Example where, as in the default `config.h`, the only string in `MATCHES` is `"RUNNING"`:

    $ change-vol-pactl +5%
      pactl set-sink-volume 1 +5%
    $ change-vol-pactl toggle
      pactl set-sink-mute 1 toggle
    $ pactl list sinks short
      0	alsa_output.pci-00[...]6le 2ch 44100Hz	IDLE
      1	alsa_output.pci-00[...]2ch 48000Hz	RUNNING

## Installation

    $ git clone https://github.com/deivshon/change-vol-pactl
    $ cd change-vol-pactl
    # make clean install

## Removal
    # make clean uninstall
