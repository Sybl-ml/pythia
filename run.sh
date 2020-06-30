#!/bin/sh

# Enter the pythia repository
cd ~/pythia

while true; do
	# Ensure we are up to date and get the current commit hash
	git checkout master
	git pull origin master
	curr=$(git rev-parse HEAD)

	# Build a release binary and start the bot in the background
	/root/.cargo/bin/cargo build --release
	/root/.cargo/bin/cargo run --release &

	# Store the process identifier of the bot
	pid=$!

	while true; do
		# Sleep to wait for changes
		sleep 30m

		# Fetch to see if changes have been made
		git fetch
		# Get the upstream commit hash
		upstream=$(git rev-parse @{u})

		# If the hashes aren't the same, updates exist
		if [ $curr != $upstream ]; then
			# Kill the currently running process and return to the start of the script
			kill $pid
			break
		fi
	done
done
