#!/bin/bash
chmod 777 /usr/src/emtm/scripts/wait-for-it.sh
bash /usr/src/emtm/scripts/wait-for-it.sh db:3306 -- diesel setup --migration-dir /usr/src/emtm/emtm-db && ./target/release/emtm-web


