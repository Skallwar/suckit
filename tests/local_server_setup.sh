#!/bin/sh

set -e

# Create the local's server directory if necessary
if [[ ! -d local_server ]]; then
    mkdir local_server
fi

cd local_server

# Clone repository if necessary
if [[ ! -d linux ]]; then
    # Clone a big repository
    git clone https://github.com/torvalds/linux

    cd linux

    # Checkout the 5.9 release
    git checkout v5.9

    # No need for the .git directory. It creates a ton of files that take too long
    # to scrape
    rm -rf .git

    # Get back to local_server
    cd ..
fi

# Start up the local python server. Choosing port 80 requires sudo privileges
sudo python3 -m http.server 80
