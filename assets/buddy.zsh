BUDDY_CMD=""
BUDDY_START=0

preexec() {
  BUDDY_CMD="$1"
  BUDDY_START=$(date +%s%3N)
}

precmd() {
  if [[ -z "$BUDDY_CMD" ]]; then
    return
  fi

  local exit_code=$?
  local end_time=$(date +%s%3N)
  local duration=$((end_time - BUDDY_START))

  buddysh \
    "$BUDDY_CMD" \
    "$exit_code" \
    "$duration" \
    "$end_time"

  BUDDY_CMD=""
  BUDDY_START=0
}