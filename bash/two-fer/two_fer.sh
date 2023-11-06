#!/usr/bin/env bash

main() {
  if [ -z "$1" ]; then
    subject="you"
  else
    subject=$1
  fi

  echo "One for $subject, one for me."
}

main "$@"
