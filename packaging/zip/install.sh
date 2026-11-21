#!/bin/sh
# Installer for the Marshmellow Linux zip.
#
# Installs for the current user by default (no password needed), or for every user with
# --system. When a desktop session and zenity or kdialog are available it shows dialogs;
# otherwise it prints to the terminal. Nothing is installed until you confirm.
set -eu

HERE=$(cd "$(dirname "$0")" && pwd)
SRC="$HERE/usr"
TITLE="Marshmellow installer"

usage() {
	cat <<'EOF'
Usage: ./install.sh [--user | --system] [--uninstall] [--no-gui] [--yes]

  --user       Install for the current user under ~/.local (default, no password)
  --system     Install for all users under /usr/local (asks for an administrator password)
  --uninstall  Remove a previous install made by this script
  --no-gui     Never show dialogs, even in a desktop session
  --yes        Do not ask for confirmation
EOF
}

MODE=user
ACTION=install
GUI=auto
ASSUME_YES=0
for arg in "$@"; do
	case $arg in
	--user) MODE=user ;;
	--system) MODE=system ;;
	--uninstall) ACTION=uninstall ;;
	--no-gui) GUI=off ;;
	--yes | -y) ASSUME_YES=1 ;;
	-h | --help)
		usage
		exit 0
		;;
	*)
		echo "Unknown option: $arg" >&2
		usage >&2
		exit 2
		;;
	esac
done

# A dialog tool is used only inside a graphical session.
DIALOG=
if [ "$GUI" = auto ] && { [ -n "${DISPLAY:-}" ] || [ -n "${WAYLAND_DISPLAY:-}" ]; }; then
	if command -v zenity >/dev/null 2>&1; then
		DIALOG=zenity
	elif command -v kdialog >/dev/null 2>&1; then
		DIALOG=kdialog
	fi
fi

info() {
	printf '%s\n' "$1"
	case $DIALOG in
	zenity) zenity --info --no-markup --width=420 --title="$TITLE" --text="$1" 2>/dev/null || true ;;
	kdialog) kdialog --title "$TITLE" --msgbox "$1" 2>/dev/null || true ;;
	esac
}

fail() {
	printf 'Error: %s\n' "$1" >&2
	case $DIALOG in
	zenity) zenity --error --no-markup --width=420 --title="$TITLE" --text="$1" 2>/dev/null || true ;;
	kdialog) kdialog --title "$TITLE" --error "$1" 2>/dev/null || true ;;
	esac
	exit 1
}

# Returns 0 for yes. Without a dialog, asks on the terminal when there is one.
confirm() {
	[ "$ASSUME_YES" = 1 ] && return 0
	case $DIALOG in
	zenity) zenity --question --no-markup --width=420 --title="$TITLE" --text="$1" 2>/dev/null ;;
	kdialog) kdialog --title "$TITLE" --yesno "$1" 2>/dev/null ;;
	*)
		[ -t 0 ] || return 0
		printf '%s [Y/n] ' "$1"
		read -r answer
		case $answer in [nN]*) return 1 ;; *) return 0 ;; esac
		;;
	esac
}

# System-wide changes need root. Re-run this script as root without dialogs, then report here.
if [ "$MODE" = system ] && [ "$(id -u)" != 0 ]; then
	confirm "Install Marshmellow for all users? An administrator password will be requested." ||
		exit 0
	if [ "$ACTION" = uninstall ]; then ROOT_ARGS="--system --uninstall"; else ROOT_ARGS="--system"; fi
	# shellcheck disable=SC2086 # ROOT_ARGS is two fixed words.
	if [ -n "$DIALOG" ] && command -v pkexec >/dev/null 2>&1; then
		AS_ROOT="pkexec"
	elif command -v sudo >/dev/null 2>&1; then
		AS_ROOT="sudo"
	else
		fail "System-wide install needs pkexec or sudo, and neither is available. Use --user instead."
	fi
	# shellcheck disable=SC2086
	if $AS_ROOT sh "$HERE/install.sh" $ROOT_ARGS --no-gui --yes; then
		info "Done. Marshmellow is available to all users."
		exit 0
	fi
	fail "The system-wide step did not complete."
fi

if [ "$MODE" = system ]; then
	BIN=/usr/local/bin
	DATA=/usr/local/share
else
	BIN="$HOME/.local/bin"
	DATA="${XDG_DATA_HOME:-$HOME/.local/share}"
fi
DESKTOP_FILE="$DATA/applications/cz.viceverse.serein.desktop"

refresh_caches() {
	command -v update-desktop-database >/dev/null 2>&1 &&
		update-desktop-database "$DATA/applications" >/dev/null 2>&1 || true
	command -v gtk-update-icon-cache >/dev/null 2>&1 &&
		gtk-update-icon-cache -q -t -f "$DATA/icons/hicolor" >/dev/null 2>&1 || true
}

if [ "$ACTION" = uninstall ]; then
	confirm "Remove Marshmellow from $BIN and $DATA?" || exit 0
	rm -f "$BIN/serein" "$DESKTOP_FILE"
	if [ -d "$SRC/share/icons/hicolor" ]; then
		(cd "$SRC/share/icons/hicolor" && find . -type f) | while IFS= read -r icon; do
			rm -f "$DATA/icons/hicolor/$icon"
		done
	fi
	rm -rf "$DATA/doc/serein"
	refresh_caches
	info "Marshmellow was removed. Your account data and settings were left in place."
	exit 0
fi

[ -x "$SRC/bin/serein" ] ||
	fail "Cannot find usr/bin/serein next to this script. Unzip the whole archive and run install.sh from inside it."

# The zip is built on a recent Ubuntu, so an older distribution may lack some libraries.
MISSING=$(ldd "$SRC/bin/serein" 2>/dev/null | sed -n 's/^[[:space:]]*\(.*\) => not found.*/\1/p')
if [ -n "$MISSING" ]; then
	confirm "These libraries are missing on this system, so Marshmellow will not start until they are installed:

$MISSING

Install anyway?" || exit 0
fi

confirm "Install Marshmellow?

Program: $BIN/serein
Menu entry and icons: $DATA" || exit 0

mkdir -p "$BIN" "$DATA/applications" "$DATA/icons/hicolor"
install -m 0755 "$SRC/bin/serein" "$BIN/serein"
# An absolute Exec path keeps the menu entry working even when ~/.local/bin is not on PATH.
sed "s|^Exec=.*|Exec=$BIN/serein|" "$SRC/share/applications/cz.viceverse.serein.desktop" >"$DESKTOP_FILE"
chmod 0644 "$DESKTOP_FILE"
cp -R "$SRC/share/icons/hicolor/." "$DATA/icons/hicolor/"
if [ -d "$SRC/share/doc/serein" ]; then
	mkdir -p "$DATA/doc"
	rm -rf "$DATA/doc/serein"
	cp -R "$SRC/share/doc/serein" "$DATA/doc/serein"
fi
refresh_caches

NOTE=""
case ":$PATH:" in
*":$BIN:"*) ;;
*) [ "$MODE" = user ] && NOTE="

$BIN is not on your PATH, so start it from the app menu or run $BIN/serein." ;;
esac
info "Marshmellow is installed. Look for it in your app menu.$NOTE"
