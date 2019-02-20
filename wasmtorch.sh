#!/bin/sh

# PATHS
# Get current working directory
current_dir=`pwd`

# Get the absolute path of where script is running from
src="${BASH_SOURCE[0]}"
while [ -h "$src" ]; do # resolve $src until the file is no longer a symlink
  dir="$( cd -P "$( dirname "$src" )" >/dev/null 2>&1 && pwd )"
  src="$(readlink "$src")"
  [[ $src != /* ]] && src="$dir/$src" # if $src was a relative symlink, we need to resolve it relative to the path where the symlink file was located
done
script_dir="$(cd -P "$(dirname "$src" )" >/dev/null 2>&1 && pwd)"
script_path="$script_dir/setup.sh"

# RETURN VARIABLE
ret=""

# ARGUMENTS
args="${@:2}" # All arguments except the first

# DESCRIPTION:
#	Where execution starts
main() {
    case $1 in
        install )
            install $2
        ;;
        uninstall )
            uninstall
        ;;
        --help|help|-h )
            help
        ;;
    esac

    exit 0
}

# DESCRIPTION:
#	Prints helpful information about the setup script
help() {
    echo ""
    echo " ========================= WASMTORCH ============================"
    echo "|                                                                |"
    echo "| [USAGE] : wasmtorch [comand]                                   |"
    echo "| [COMMAND] :                                                    |"
    echo "|  • help       - prints this help message                       |"
    echo "|  • install    - builds project and exposes relevant commands   |"
    echo "|  • uninstall  - removes build files and commands               |"
    echo "|                                                                |"
    echo " ================================================================"
    echo ""
}

# TODO: Debug install (wasmlited) vs release install (wasmlite)
# DESCRIPTION:
#	Installs wasmlite project
install() {
    local wasmlite_path="$script_dir/target/debug/wasmlite"
    local usr_prefix="/usr/local/bin"

    #--------------------------------------------------

    # TODO: Seperate release build.
    displayln "Build wasmlite project"
    # Build cargo project.
    LLVM_SYS_60_PREFIX=$1 cargo build --feature "verbose"

    #--------------------------------------------------

    displayln "Make commands accessible system-wide"
    # Make setup script executable
    chmod u+x $script_path

    # Add links to commands in /usr/local/bin
    if [ ! -f "$usr_prefix/wasmlite" ]; then
        add_link "wasmlite" $wasmlite_path
    fi

    if [ ! -f "$usr_prefix/wasmtorch" ]; then
        add_link "wasmtorch" $script_path
    fi
}

# TODO: Refactor
# DESCRIPTION:
#	Uninstalls wasmlite project
uninstall() {
    if confirm "uninstall wasmlite"; then
        echo "Exiting"
        exit 0
    fi

    # TODO
    #---------------- Remove cargo build --------------
    #--------------------------------------------------

    displayln "Remove commands"
    remove_link "wasmlite"
    remove_link "wasmtorch"
}

# DESCRIPTION:
#	Adds a symbolic link to files in `/usr/local/bin`
add_link() {
    if [ -z $1 ]; then
        echo "You need to specify link name!"
        exit 1
    fi

    if [ -z $2 ]; then
        echo "You need to specify the file you want to link to!"
        exit 1
    fi

    # displayln "Add a link to specified file in /usr/local/bin"
    ln -s $2 /usr/local/bin/$1
}

# DESCRIPTION:
#   Removes a symbolic link from `/usr/local/bin`
remove_link() {
    if [ -z $1 ]; then
        echo "You need to provide the symbolic file to delete!"
        exit 1
    fi

    # displayln "Check that file is a link"
    if [ ! -L "/usr/local/bin/$1" ]; then
        echo "What you specified is not a symbolic link!"
        exit 1
    fi

    # displayln "Remove link `/usr/local/bin`"
    rm /usr/local/bin/$1
}

# DESCRIPTION:
#	Displays a message with multiple trainling newlines
displayln() {
    printf "\n:::: $1 ::::\n\n"
}

# DESCRIPTION:
#	Displays a message
display() {
    printf "\n:::: $1 ::::\n"
}

# DESCRIPTION:
#	Asks the user for confirmation befor proceeding
confirm() {
	printf "\n:::: Are you sure you want to $1? [Y/n] "

	read response

	if [ $response = "Y" ]; then
		return 1
	else
		return 0
	fi
}

# Start main
main $@
