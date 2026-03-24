#! /usr/bin/bash

cargo --version

prek self update
prek auto-update --cooldown-days=31
prek install
