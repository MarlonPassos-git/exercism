#!/usr/bin/env bash

main() {
  isFactorOf3=$(isFactor "$1" 3)
  isFactorOf5=$(isFactor "$1" 5)
  isFactorOf7=$(isFactor "$1" 7)
  result=""

  if [ "$isFactorOf3" -eq 0 ]; then
    result+="Pling"
  fi
  if [ "$isFactorOf5" -eq 0 ]; then
    result+="Plang"
  fi
  if [ "$isFactorOf7" -eq 0 ]; then
    result+="Plong"
  fi

  if [ -z $result ]; then
    echo "$1"
  fi

  echo $result
}

isFactor() {
  if [ $(($1 % $2)) -eq 0 ]; then
    echo 0
  else
    echo 1
  fi
}

main "$@"
