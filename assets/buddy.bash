buddy_preexec() {
  BUDDY_CMD="$BASH_COMMAND"
  BUDDY_START=$(date +%s%3N)
}

buddy_precmd() {
  local exit_code=$?
  local end_time=$(date +%s%3N)
  local duration=$((end_time - BUDDY_START))

  buddysh \
    "$BUDDY_CMD" \
    "$exit_code" \
    "$duration" \
    "$end_time" &
}

trap buddy_preexec DEBUG
PROMPT_COMMAND=buddy_precmd