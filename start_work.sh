#! /bin/sh

tmux new-session -s MovieThingy \; \
 split-window -h -p '65' \; \
 send-keys -t 0 'cd ~/MovieThingy && cargo check' Enter \; \
 send-keys -t 1 'cd ~/MovieThingy; vim' Enter \;



