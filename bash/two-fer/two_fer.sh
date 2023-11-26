#!/usr/bin/env bash

main() {
  subject=${1:-"you"}

  echo "One for $subject, one for me."
}

main "$@"
