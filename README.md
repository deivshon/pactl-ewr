# change-vol-pactl
This wrapper utility provides control over PulseAudio sinks using the pactl command-line tool: it allows to toggle mute status and change volume on sinks that match a specified string

Matching is done against the output lines of the command `pactl list sinks short`

## Options

| Short Argument  | Long Argument    | Default        | Description                                                                                   |
|-----------------|------------------|----------------|-----------------------------------------------------------------------------------------------|
| `-t`            | `--toggle`       | `false`        | Toggle mute on matching sinks                                                                 |
| `-c`            | `--volume-change`| `0`            | Volume change percentage on matching sinks                                                    |
| `-n`            | `--needle`       | `RUNNING`      | Substring that triggers matching on `pactl list sinks short` output lines                     |
| `-v`            | `--verbose`      | `false`        | Verbose output                                                                                |

## Examples

- Increase volume by 10% on sinks with "IDLE" in the corresponding line:
```sh
$ pactl-ewr -c 10 -n IDLE
```

- Show verbose output while toggling mute on sinks with "ACTIVE" in the corresponding line:
```sh
$ pactl-ewr -t -n ACTIVE -v
```
