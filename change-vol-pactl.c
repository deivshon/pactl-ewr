#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include "config.h"

#define printerr(str) fprintf(stderr, str);

#define PACTL_COMMAND   "pactl"
#define CHANGE_VOL_ARG  "set-sink-volume"
#define TOGGLE_VOL_ARG  "set-sink-mute"

int pactl_exists() {
    struct stat buf;
    return stat("/usr/bin/pactl", &buf) == 0 || stat("/usr/local/bin/pactl", &buf) == 0;
}

void output_exec(char *dest, int dest_size, char *cmd, char **cmd_args) {
    int piped[2];
    if(pipe(piped) == -1) {
        printerr("Couldn't initiate pipe\n");
        exit(EXIT_FAILURE);
    }

    if(fork() == 0) {
        close(piped[0]);
        dup2(piped[1], 1);
        dup2(piped[1], 2);
        close(piped[1]);

        execvp(cmd, cmd_args);
        printerr("Exec failed");
        exit(EXIT_FAILURE);
    }
    else {
        dest[0] = '\0';

        int c = 0;
        char buf[1024] = "";
        close(piped[1]);

        while((c = read(piped[0], buf, sizeof(buf) - 1))) {
            // Sets the byte after the last one read to '\0', terminating the string
            buf[c] = '\0';

            if(strlen(dest) + strlen(buf) >= (size_t) dest_size)
                break;

            strcat(dest, buf);
        }
        close(piped[0]);
    }
}

void match_call(char *line, char *vol_arg, char *vol) {
    char *endptr_line;
    char *sink_num = strtok_r(line, "\t\r ", &endptr_line);

    char *pactl_args[5] = {PACTL_COMMAND, vol_arg, sink_num, vol, NULL};
    printf("%s %s %s %s\n", pactl_args[0], pactl_args[1], pactl_args[2], pactl_args[3]);

    char pactl_output[256];
    output_exec(pactl_output, sizeof(pactl_output) - 1, PACTL_COMMAND, pactl_args);

    printf("%s", pactl_output);
}

int main(int argc, char **argv) {
    if(!pactl_exists()) {
        printerr("Pactl does not seem to be installed\n");
        exit(EXIT_FAILURE);
    }
    if(argc < 2) {
        printerr("No argument provided\n");
        exit(EXIT_FAILURE);
    }

    char *vol_arg = strcmp("toggle", argv[1]) ? CHANGE_VOL_ARG : TOGGLE_VOL_ARG;
    char *vol = argv[1];

    char *pactl_list_sinks[5] = {PACTL_COMMAND, "list", "sinks", "short", NULL};
    char sinks_list[2048];
    output_exec(sinks_list, sizeof(sinks_list) - 1, PACTL_COMMAND, pactl_list_sinks);

    char *matches[MATCHES_LEN] = MATCHES;
    char *endptr_whole;
    char *line = strtok_r(sinks_list, "\n", &endptr_whole);
    while(line != NULL) {
        for(int i = 0; i < MATCHES_LEN; i++) {
            if(strstr(line, matches[i])) {
                match_call(line, vol_arg, vol);
                wait(NULL);
                break;
            }
        }

        line = strtok_r(NULL, "\n", &endptr_whole);
    }
}
