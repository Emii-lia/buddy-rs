function buddy_preexec --on-event fish_preexec
  set -g BUDDY_CMD "$argv"
  set -g BUDDY_START (date +%s%3N)
end

function buddy_postexec --on-event fish_postexec
  set -l exit_code $status
  set -l end_time (date +%s%3N)
  set -l duration (math "$end_time - $BUDDY_START")

  command buddysh \
    "$BUDDY_CMD" \
    "$exit_code" \
    "$duration" \
    "$end_time"
end