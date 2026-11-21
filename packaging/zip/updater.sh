#!/bin/sh
# Update a zip installation of Marshmellow.
set -eu

HERE=$(cd "$(dirname "$0")" && pwd)
MODE=user
GUI=auto

ZIP=
for arg in "$@"; do
	case "$arg" in
	--user) MODE=user ;;
	--system) MODE=system ;;
	--no-gui) GUI=off ;;
	-h|--help)
		echo "Usage: ./updater.sh [--user | --system] [--no-gui]"
		exit 0
		;;
	-*) echo "Unknown option: $arg" >&2; exit 2 ;;
	*) ZIP="$arg" ;;
	esac
done

if [ "$MODE" = system ] && [ "$(id -u)" -ne 0 ]; then
	echo "Requesting administrator access..."
	if command -v pkexec >/dev/null 2>&1; then
		exec pkexec sh "$HERE/updater.sh" --system --no-gui
	elif command -v sudo >/dev/null 2>&1; then
		exec sudo sh "$HERE/updater.sh" --system --no-gui
	else
		echo "System update requires pkexec or sudo." >&2
		exit 1
	fi
fi

if [ -n "${MARSHMELLOW_ZIP:-}" ]; then
	ZIP="$MARSHMELLOW_ZIP"
elif [ -z "$ZIP" ]; then
	echo "Set MARSHMELLOW_ZIP to the replacement zip, or pass it as the first argument." >&2
	exit 2
fi

[ -f "$ZIP" ] || { echo "Replacement zip not found: $ZIP" >&2; exit 1; }

TMP=$(mktemp -d "${TMPDIR:-/tmp}/marshmellow-update.XXXXXX")
trap 'rm -rf "$TMP"' EXIT HUP INT TERM

unzip -q "$ZIP" -d "$TMP"
EXTRACTED=$(find "$TMP" -type f -name install.sh -print -quit)
[ -n "$EXTRACTED" ] || { echo "The zip does not contain install.sh." >&2; exit 1; }
ROOT=$(dirname "$EXTRACTED")
exec sh "$ROOT/install.sh" "--$MODE" --yes
