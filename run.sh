#!/bin/sh

LOG_DIR=/root/.logs
RUN_LOG="$LOG_DIR/run.log"
PYTHIA_LOG="$LOG_DIR/pythia.log"

# Ensure the log directory and files exist
mkdir -p $LOG_DIR
touch $RUN_LOG $PYTHIA_LOG

# Logs information about the execution of this script to /root/.logs/run.log
function log {
	echo "$(date "+%Y-%m-%d %H:%M:%S") > $1" >> $RUN_LOG
}

# Enter the pythia repository
cd ~/pythia
log "Beginning $0 with logs in $LOG_DIR"

while true; do
	# Swap to the master branch if we aren't already on it
	git checkout master

	# Check for a server restart required (if this file has changed)
	timestamp=$(date +%s -r run.sh)
	log "Current run.sh timestamp: $timestamp"

	# Pull the most recent changes
	log "Pulling recent changes"
	git pull origin master

	possibly_updated=$(date +%s -r run.sh)
	log "New run.sh timestamp: $possibly_updated"

	if [ $timestamp -lt $possibly_updated ]; then
		log "run.sh has updated, restarting the server"
		sudo shutdown -r now
	fi

	curr=$(git rev-parse HEAD)
	log "Updated to commit hash: $curr"

	# Build a release binary and start the bot in the background
	/root/.cargo/bin/cargo build --release
	/root/.cargo/bin/cargo run --release >> $PYTHIA_LOG 2>&1 &

	# Store the process identifier of the bot
	pid=$!
	log "Began executing Pythia with pid: $pid"

	while true; do
		# Sleep to wait for changes
		sleep 30m

		# Fetch to see if changes have been made
		git fetch
		# Get the upstream commit hash
		upstream=$(git rev-parse @{u})
		log "Upstream commit: $upstream"

		# If the hashes aren't the same, updates exist
		if [ $curr != $upstream ]; then
			# Kill the currently running process and return to the start of the script
			log "Hashes are different, curr: '$curr' vs upstream: '$upstream'"
			kill $pid
			log "Killed process with pid: $pid"
			break
		fi
	done
done
