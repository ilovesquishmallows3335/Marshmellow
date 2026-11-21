#!/bin/sh
# Uninstall a zip installation of Marshmellow.
set -eu

HERE=$(cd "$(dirname "$0")" && pwd)
MODE=user

for arg in "$@"; do
	case "$arg" in
	--user) MODE=user ;;
	--system) MODE=system ;;
	-h|--help)
		echo "Usage: ./uninstall.sh [--user | --system]"
		exit 0
		;;
	*) echo "Unknown option: $arg" >&2; exit 2 ;;
	esac
done

exec sh "$HERE/install.sh" "--$MODE" --uninstall --yes
